//! Select a desktop bundle or an already-installed app on an explicit simulator.
use crate::{Error, Result, android, require, rpc, validator::Validator};
use leptoskit_testing::native::{Application, NativeTester, Options, Platform};
use serde_json::json;
use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

const DRIVER_STARTUP_SECONDS: u64 = 900;
const UI_WAIT_SECONDS: u64 = 30;
const APP_NOT_RUNNING: u64 = 1;
const USAGE: &str = "usage: installed_wallet <macos|ios|android> <loopback-driver> <application> <fresh-config-directory> <fresh-run-directory> <validator-binary> [<device> <prebuilt-wda-directory|adb-binary>]";

enum Target {
    Macos(String),
    Ios {
        identifier: String,
        device: String,
        wda: String,
    },
    Android {
        identifier: String,
        device: String,
        adb: PathBuf,
    },
}

pub(crate) struct Launch {
    target: Target,
    driver: String,
    pub data: PathBuf,
    pub directory: PathBuf,
    pub validator: OsString,
}

impl Launch {
    pub(crate) fn read() -> Result<Self> {
        Self::parse(&std::env::args_os().skip(1).collect::<Vec<_>>())
    }

    fn parse(arguments: &[OsString]) -> Result<Self> {
        let [
            platform,
            driver,
            app,
            data,
            directory,
            validator,
            extra @ ..,
        ] = arguments
        else {
            return Err(Error::Configuration(USAGE.into()));
        };
        let driver = text(driver)?;
        rpc::loopback_url(&driver)?;

        let target = match (platform.to_str(), extra) {
            (Some("macos"), []) => {
                Target::Macos(text(&PathBuf::from(app).canonicalize()?.into_os_string())?)
            }
            (Some("ios"), [device, wda]) => Target::Ios {
                identifier: text(app)?,
                device: text(device)?,
                wda: text(&PathBuf::from(wda).canonicalize()?.into_os_string())?,
            },
            (Some("android"), [device, adb]) => {
                let identifier = text(app)?;
                android::validate_package(&identifier)?;
                Target::Android {
                    identifier,
                    device: text(device)?,
                    adb: PathBuf::from(adb).canonicalize()?,
                }
            }
            _ => return Err(Error::Configuration(USAGE.into())),
        };

        Ok(Self {
            target,
            driver,
            data: data.into(),
            directory: directory.into(),
            validator: validator.clone(),
        })
    }

    pub(crate) fn configure(&self, node: &Validator, data: &Path) -> Result<()> {
        let host = if matches!(&self.target, Target::Android { .. }) {
            "10.0.2.2"
        } else {
            "127.0.0.1"
        };

        let bytes = serde_json::to_vec(&node.guest_endpoints(host))?;
        fs::write(data.join("journey.json"), &bytes)?;

        if let Target::Android {
            identifier,
            device,
            adb,
        } = &self.target
        {
            android::configure(adb, device, identifier, &bytes)?;
        }

        Ok(())
    }

    /// Explicitly terminate this app, then require the OS driver's not-running
    /// state before another session may claim to exercise a process relaunch.
    pub(crate) fn stop_app(&self, tester: &NativeTester) -> Result<()> {
        let (driver, arguments) = match &self.target {
            Target::Macos(path) => ("macos", json!({"path": path})),
            Target::Ios { identifier, .. } => ("mobile", json!({"bundleId": identifier})),
            Target::Android { identifier, .. } => ("mobile", json!({"appId": identifier})),
        };

        let stopped = tester.native_command(&format!("{driver}: terminateApp"), &arguments)?;

        let acknowledged = match &self.target {
            Target::Macos(_) => stopped.is_null(),
            Target::Ios { .. } | Target::Android { .. } => stopped == true,
        };

        require(acknowledged, "driver did not acknowledge app termination")?;
        let state = tester.native_command(&format!("{driver}: queryAppState"), &arguments)?;
        require(
            state.as_u64() == Some(APP_NOT_RUNNING),
            "app still running before relaunch",
        )
    }

    /// Reuse installed mobile apps so their prelaunch RPC configuration survives the session.
    pub(crate) fn options(&self) -> Options {
        let (platform, application, device) = match &self.target {
            Target::Macos(path) => (Platform::Macos, Application::Path(path.clone()), None),
            Target::Ios {
                identifier, device, ..
            } => (
                Platform::Ios,
                Application::Identifier(identifier.clone()),
                Some(device.clone()),
            ),
            Target::Android {
                identifier, device, ..
            } => (
                Platform::Android,
                Application::Identifier(identifier.clone()),
                Some(device.clone()),
            ),
        };

        let mut options = Options::new(&self.driver, platform, application);
        options.device = device;

        if matches!(platform, Platform::Ios | Platform::Android) {
            for capability in ["appium:forceAppLaunch", "appium:shouldTerminateApp"] {
                options.capabilities.insert(capability.into(), true.into());
            }
        }

        if let Target::Ios { wda, .. } = &self.target {
            options
                .capabilities
                .insert("appium:usePrebuiltWDA".into(), true.into());
            options
                .capabilities
                .insert("appium:derivedDataPath".into(), wda.clone().into());
            options
                .capabilities
                .insert("appium:showXcodeLog".into(), true.into());
        }

        if platform == Platform::Android {
            options
                .capabilities
                .insert("appium:appActivity".into(), ".MainActivity".into());
        }

        options.artifacts = self.directory.join("native-ui");
        options.startup_seconds = DRIVER_STARTUP_SECONDS;
        options.wait_seconds = UI_WAIT_SECONDS;
        options
    }
}

fn text(value: &OsString) -> Result<String> {
    let text = value
        .to_str()
        .ok_or_else(|| Error::Configuration("session identity must be UTF-8".into()))?;
    require(
        !text.trim().is_empty(),
        "session identity must not be empty",
    )?;
    Ok(text.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mobile() -> Vec<OsString> {
        [
            "ios",
            "http://127.0.0.1:14731",
            "com.leptoskit.solanawalletapp",
            "fresh-data",
            "fresh-run",
            "validator",
            "explicit-simulator",
            ".",
        ]
        .map(OsString::from)
        .into()
    }

    #[test]
    fn ios_launch_uses_the_installed_identity_without_reinstalling_its_data() {
        let launch = Launch::parse(&mobile()).unwrap();
        let options = launch.options();
        let expected_identifier = "com.leptoskit.solanawalletapp";
        assert!(matches!(options.platform, Platform::Ios));
        assert!(
            matches!(options.application, Application::Identifier(ref id) if id == expected_identifier)
        );
        assert_eq!(options.device.as_deref(), Some("explicit-simulator"));
        assert_eq!(options.capabilities["appium:shouldTerminateApp"], true);
        assert_eq!(options.capabilities["appium:usePrebuiltWDA"], true);
    }

    #[test]
    fn mobile_requires_a_device_and_a_numeric_local_driver() {
        let mut arguments = mobile();
        arguments.truncate(6);
        assert!(Launch::parse(&arguments).is_err());
        arguments = mobile();
        arguments[1] = "http://example.invalid:14731".into();
        assert!(Launch::parse(&arguments).is_err());
    }
}
