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
mod wallet;

use leptoskit_testing::native::{Error, NativeTester, Platform};
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
    let result = match launch.wallet() {
        Some(wallet) => external(&launch, &mut node, platform, wallet),
        None => local(&launch, &mut node, platform),
    };

    finish([
        result,
        startup::retain(data.path(), &launch.directory).map_err(Into::into),
        node.close(),
        data.close(),
    ])
}

/// The maintained local-wallet journey remains the default controller path.
fn local(
    launch: &launch::Launch,
    node: &mut validator::Validator,
    platform: Platform,
) -> Result<()> {
    let wallets = NativeTester::run(launch.options(), |tester| {
        let wallets = journey::run(tester, node, platform)?;
        launch.stop_app(tester)?;

        Ok(wallets)
    })?;
    NativeTester::run(launch.options(), |tester| {
        wallets.saved.verify(tester, node, platform)?;
        launch.stop_app(tester)
    })?;
    NativeTester::run(launch.options(), |tester| {
        wallets.backup.verify(tester, node, platform)?;
        launch.stop_app(tester)
    })
}

/// The generated application's ordinary MWA path. Only the selected public
/// package metadata is retained; the wallet's private storage is never read.
fn external(
    launch: &launch::Launch,
    node: &mut validator::Validator,
    platform: Platform,
    wallet: &str,
) -> Result<()> {
    record_wallet(launch, wallet)?;
    let external = external_context(launch)?;
    let state = initial_session(launch, node, platform, &external)?;

    relaunch_session(launch, platform, &external, &state)
}

/// Records the installed wallet's own package, version and code. A missing
/// installation remains an environment prerequisite rather than a code failure.
fn record_wallet(launch: &launch::Launch, wallet: &str) -> Result<()> {
    let (adb, device) = launch
        .adb()
        .ok_or_else(|| Error::Configuration("MWA acceptance requires Android".into()))?;
    let metadata = wallet::inspect(adb, device, wallet)?;
    let record = serde_json::json!({
        "package": metadata.package,
        "version_name": metadata.version_name,
        "version_code": metadata.version_code,
    });
    let bytes = serde_json::to_vec_pretty(&record)?;
    fs::write(launch.directory.join("wallet.json"), bytes)?;

    Ok(())
}

fn external_context(launch: &launch::Launch) -> Result<journey::mobile::External<'_>> {
    let (adb, device) = launch
        .adb()
        .ok_or_else(|| Error::Configuration("MWA acceptance requires Android".into()))?;
    let app = launch
        .app_identifier()
        .ok_or_else(|| Error::Configuration("MWA acceptance requires an Android app".into()))?;

    Ok(journey::mobile::External { app, adb, device })
}

/// The first process owns all approving, rejecting and cancellation evidence.
/// Its app and wallet are terminated before the second process starts.
fn initial_session(
    launch: &launch::Launch,
    node: &mut validator::Validator,
    platform: Platform,
    external: &journey::mobile::External<'_>,
) -> Result<journey::mobile::State> {
    NativeTester::run(launch.options(), |tester| {
        let state = journey::mobile::initial(tester, node, platform, external)?;
        launch.stop_app(tester)?;
        launch.stop_wallet(tester)?;

        Ok(state)
    })
}

/// A genuine process relaunch must authorize the same public account again.
fn relaunch_session(
    launch: &launch::Launch,
    platform: Platform,
    external: &journey::mobile::External<'_>,
    state: &journey::mobile::State,
) -> Result<()> {
    NativeTester::run(launch.options(), |tester| {
        journey::mobile::relaunch(tester, platform, external, state)?;
        launch.stop_app(tester)?;
        launch.stop_wallet(tester)
    })
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
