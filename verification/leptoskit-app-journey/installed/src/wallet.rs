//! The external wallet is an independently built, already-installed application.
//! Only its public package metadata and foreground task are used.
use crate::{Error, Result, require};
use leptoskit_testing::native::{Locator, NativeTester};
use serde_json::json;
use std::{
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(30);
const POLL: Duration = Duration::from_millis(25);
const APP_SWITCH: i64 = 187;

pub(crate) struct Metadata {
    pub package: String,
    pub version_name: String,
    pub version_code: u64,
}

/// Reads the installed package's own version. A missing wallet is an explicit
/// environment prerequisite, never a code failure disguised as one.
pub(crate) fn inspect(adb: &Path, device: &str, package: &str) -> Result<Metadata> {
    let output = command(
        adb,
        device,
        &["shell", "dumpsys", "package", package],
        TIMEOUT,
    )?;
    let text = String::from_utf8(output)
        .map_err(|_| Error::Response("wallet package metadata is not UTF-8".into()))?;
    parse(package, &text)
}

fn command(adb: &Path, device: &str, arguments: &[&str], timeout: Duration) -> Result<Vec<u8>> {
    let path = std::env::temp_dir().join(format!(
        "solana-kit-wallet-{}-{}.txt",
        std::process::id(),
        arguments.last().unwrap_or(&"inspect")
    ));
    let output = std::fs::File::create(&path)?;
    let mut child = Command::new(adb)
        .args(["-s", device])
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::from(output))
        .stderr(Stdio::null())
        .spawn()?;
    let deadline = Instant::now() + timeout;

    loop {
        if let Some(status) = child.try_wait()? {
            let bytes = std::fs::read(&path)?;
            let _ = std::fs::remove_file(&path);
            require(status.success(), "wallet package inspection failed")?;

            return Ok(bytes);
        }

        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            let _ = std::fs::remove_file(&path);

            return Err(Error::Response(
                "wallet package inspection deadline".into(),
            ));
        }

        thread::sleep(POLL);
    }
}

fn parse(package: &str, text: &str) -> Result<Metadata> {
    require(
        text.contains(&format!("Package [{package}]")),
        "external wallet package is not installed",
    )?;
    let version_name = field(text, "versionName=")?;
    let version_code = field(text, "versionCode=")?
        .split_whitespace()
        .next()
        .ok_or_else(|| Error::Response("wallet version code is missing".into()))?
        .parse::<u64>()
        .map_err(|error| Error::Response(error.to_string()))?;

    Ok(Metadata {
        package: package.into(),
        version_name,
        version_code,
    })
}

fn field(text: &str, prefix: &str) -> Result<String> {
    text.lines()
        .find_map(|line| line.trim().strip_prefix(prefix))
        .map(str::to_owned)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| Error::Response(format!("wallet metadata lacks {prefix}")))
}

/// Brings the existing wallet task forward so a delayed request is displayed
/// while the wallet is resumed. This is an ordinary Android task switch.
pub(crate) fn present(tester: &NativeTester) -> Result<()> {
    tester.native_command("mobile: pressKey", &json!({"keycode": APP_SWITCH}))?;
    tester.tap(&Locator::accessibility_id("Fake Wallet app"))
}

/// Activates the generated application's own task after a wallet interaction.
pub(crate) fn app(tester: &NativeTester, package: &str) -> Result<()> {
    tester.native_command("mobile: activateApp", &json!({"appId": package}))?;
    Ok(())
}

pub(crate) fn authorize(tester: &NativeTester) -> Result<()> {
    tester.expect_visible(&Locator::text("AUTHORIZE DAPP"))?;
    tester.tap(&Locator::text("AUTHORIZE"))
}

pub(crate) fn sign(tester: &NativeTester, title: &str) -> Result<()> {
    tester.expect_visible(&Locator::text(title))?;
    tester.tap(&Locator::text("AUTHORIZE"))
}

pub(crate) fn decline(tester: &NativeTester) -> Result<()> {
    tester.expect_visible(&Locator::text("SIGN TRANSACTION(S)"))?;
    tester.tap(&Locator::text("DECLINE"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"Packages:
  Package [com.solana.mobilewalletadapter.fakewallet] (a1b2c3):
    versionCode=1 minSdk=23 targetSdk=37
    versionName=1.0
";

    #[test]
    fn reads_only_the_installed_package_version() {
        let metadata = parse("com.solana.mobilewalletadapter.fakewallet", EXAMPLE).unwrap();
        assert_eq!(metadata.version_name, "1.0");
        assert_eq!(metadata.version_code, 1);
    }

    #[test]
    fn rejects_another_or_missing_wallet_package() {
        assert!(parse("com.example.other", EXAMPLE).is_err());
        assert!(parse("com.solana.mobilewalletadapter.fakewallet", "").is_err());
        assert!(
            parse(
                "com.solana.mobilewalletadapter.fakewallet",
                "Package [com.solana.mobilewalletadapter.fakewallet]"
            )
            .is_err()
        );
    }
}
