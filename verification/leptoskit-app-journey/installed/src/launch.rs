//! Select a desktop bundle or an already-installed app on an explicit simulator.
use crate::{Error, Result, require, rpc};
use leptoskit_testing::native::{Application, Options, Platform};
use std::{ffi::OsString, path::PathBuf};

const DRIVER_STARTUP_SECONDS: u64 = 900;
const UI_WAIT_SECONDS: u64 = 30;
const USAGE: &str = "usage: installed_wallet <macos|ios> <loopback-driver> <application> <fresh-app-data> <fresh-run-directory> <validator-binary> [<ios-device>]";

enum Target {
    Macos(String),
    Ios { identifier: String, device: String },
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
            (Some("ios"), [device]) => Target::Ios {
                identifier: text(app)?,
                device: text(device)?,
            },
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

    pub(crate) fn options(&self) -> Options {
        let (platform, application, device) = match &self.target {
            Target::Macos(path) => (Platform::Macos, Application::Path(path.clone()), None),
            Target::Ios { identifier, device } => (
                Platform::Ios,
                Application::Identifier(identifier.clone()),
                Some(device.clone()),
            ),
        };
        let mut options = Options::new(&self.driver, platform, application);
        options.device = device;

        if platform == Platform::Ios {
            for capability in ["appium:forceAppLaunch", "appium:shouldTerminateApp"] {
                options.capabilities.insert(capability.into(), true.into());
            }
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
        ]
        .map(OsString::from)
        .into()
    }

    #[test]
    fn ios_launch_uses_the_installed_identity_without_reinstalling_its_data() {
        let launch = Launch::parse(&mobile()).unwrap();
        let options = launch.options();
        assert!(matches!(options.platform, Platform::Ios));
        assert!(
            matches!(options.application, Application::Identifier(ref id) if id == "com.leptoskit.solanawalletapp")
        );
        assert_eq!(options.device.as_deref(), Some("explicit-simulator"));
        assert_eq!(options.capabilities["appium:shouldTerminateApp"], true);
    }

    #[test]
    fn mobile_requires_a_device_and_a_numeric_local_driver() {
        let mut arguments = mobile();
        arguments.pop();
        assert!(Launch::parse(&arguments).is_err());
        arguments = mobile();
        arguments[1] = "http://example.invalid:14731".into();
        assert!(Launch::parse(&arguments).is_err());
    }
}
