//! Private off-chain application composition, selected by the public generator.
//!
//! The generated native UI thread installs, polls and closes this service outside
//! a Tokio runtime. Its two native workers run bounded HTTP, socket and timer jobs.
//! Closing joins those jobs and the wallet worker before dropping the runtime.
#![forbid(unsafe_code)]

mod config;
mod service;
mod startup;
mod timers;

use leptoskit_bridge::Error;
use leptoskit_bridge_ffi::app::ServiceInstaller;
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use std::path::Path;

type Host = WasmHost<HeadlessBackend>;

/// Reads explicit test grants from `journey.json` in the generated app sandbox.
/// No network access, wallet worker or runtime starts until activation.
pub fn from_app_dirs(data: &Path, _cache: &Path) -> Result<ServiceInstaller, Error> {
    let mut startup = startup::Startup::create(data).map_err(native)?;
    let endpoints = config::Endpoints::read(data)?;
    startup.record("configuration-ready").map_err(native)?;
    let data = data.to_owned();
    Ok(Box::new(move |host| {
        startup
            .record("service-activation-entered")
            .map_err(native)?;
        service::Service::install(host, &data, endpoints, startup)
            .map(|service| Box::new(service) as Box<dyn leptoskit_bridge_ffi::app::AppService>)
    }))
}

fn native(_: impl std::error::Error) -> Error {
    Error::Native
}

#[cfg(test)]
mod tests;
