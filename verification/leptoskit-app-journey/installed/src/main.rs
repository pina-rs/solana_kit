//! An OS driver operates the unmodified public-generator host against an owned node.
#![forbid(unsafe_code)]
mod accessibility;
mod application_data;
mod journey;
mod ports;
mod rpc;
mod validator;

use leptoskit_testing::native::{Application, Error, NativeTester, Options, Platform};
use std::{fs, path::PathBuf};

type Result<T> = std::result::Result<T, Error>;
const ARGUMENTS: usize = 5;
const DRIVER_STARTUP_SECONDS: u64 = 900;
const UI_WAIT_SECONDS: u64 = 30;

fn require(condition: bool, message: impl Into<String>) -> Result<()> {
    if !condition {
        return Err(Error::Response(message.into()));
    }

    Ok(())
}

/// Every path is explicit and the controller creates its run and app directories.
/// It cannot reuse a prior wallet, ledger, Solana CLI configuration or endpoint.
fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    require(
        arguments.len() == ARGUMENTS,
        "usage: installed_wallet <loopback-driver> <app.bundle> <fresh-app-data> <fresh-run-directory> <validator-binary>",
    )?;
    let driver = arguments[0]
        .to_str()
        .ok_or_else(|| Error::Configuration("driver must be UTF-8".into()))?;
    rpc::loopback_url(driver)?;
    let bundle = PathBuf::from(&arguments[1]).canonicalize()?;
    let data = PathBuf::from(&arguments[2]);
    let directory = PathBuf::from(&arguments[3]);

    fs::create_dir(&directory)?;
    let data = application_data::ApplicationData::create(data)?;
    let mut node = validator::Validator::start(&directory, &arguments[4])?;
    let config = serde_json::json!({"http":node.http(),"socket":node.socket()});
    fs::write(
        data.path().join("journey.json"),
        serde_json::to_vec(&config)?,
    )?;

    let mut options = Options::new(
        driver,
        Platform::Macos,
        Application::Path(
            bundle
                .into_os_string()
                .into_string()
                .map_err(|_| Error::Configuration("app bundle path must be UTF-8".into()))?,
        ),
    );
    options.artifacts = directory.join("native-ui");
    options.startup_seconds = DRIVER_STARTUP_SECONDS;
    options.wait_seconds = UI_WAIT_SECONDS;
    let result = NativeTester::run(options, |tester| journey::run(tester, &mut node));
    finish([result, node.close(), data.close()])
}

/// Evaluate every cleanup outcome, preserving a sole original error and reporting
/// all causes when both the journey and one or more owned cleanups fail.
fn finish(outcomes: [Result<()>; 3]) -> Result<()> {
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
