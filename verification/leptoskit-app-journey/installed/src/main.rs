//! An OS driver operates the unmodified public-generator host against an owned node.
#![forbid(unsafe_code)]
mod accessibility;
mod android;
mod application_data;
mod journey;
mod launch;

mod ports;
mod rpc;
mod startup;
mod validator;

use leptoskit_testing::native::{Error, NativeTester};
use std::fs;

type Result<T> = std::result::Result<T, Error>;

fn require(condition: bool, message: impl Into<String>) -> Result<()> {
    if !condition {
        return Err(Error::Response(message.into()));
    }

    Ok(())
}

/// Every path is explicit and the controller creates its run and app directories.
/// It cannot reuse a prior wallet, ledger, Solana CLI configuration or endpoint.
fn main() -> Result<()> {
    let launch = launch::Launch::read()?;
    fs::create_dir(&launch.directory)?;
    let data = application_data::ApplicationData::create(launch.data.clone())?;
    let mut node = validator::Validator::start(&launch.directory, &launch.validator)?;
    launch.configure(&node, data.path())?;

    let platform = launch.options().platform;
    let result = (|| {
        let wallets = NativeTester::run(launch.options(), |tester| {
            let wallets = journey::run(tester, &mut node, platform)?;
            launch.stop_app(tester)?;
            Ok(wallets)
        })?;
        NativeTester::run(launch.options(), |tester| {
            wallets.saved.verify(tester, &mut node, platform)?;
            launch.stop_app(tester)
        })?;
        NativeTester::run(launch.options(), |tester| {
            wallets.backup.verify(tester, &mut node, platform)?;
            launch.stop_app(tester)
        })
    })();
    finish([
        result,
        startup::retain(data.path(), &launch.directory).map_err(Into::into),
        node.close(),
        data.close(),
    ])
}

/// Evaluate every cleanup outcome, preserving a sole original error and reporting
/// all causes when both the journey and one or more owned cleanups fail.
fn finish(outcomes: impl IntoIterator<Item = Result<()>>) -> Result<()> {
    let mut failures = outcomes
        .into_iter()
        .filter_map(|result| result.err())
        .collect::<Vec<_>>();

    match failures.len() {
        0 => Ok(()),
        1 => Err(failures.remove(0)),
        _ => Err(Error::Response(
            failures
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        )),
    }
}
