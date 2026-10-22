//! Executes actual interpreted HTTP/TLS rather than a substitute interpreter.
#![forbid(unsafe_code)]
mod fixture;
mod imports;
mod journeys;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    let path = std::env::args_os()
        .nth(1)
        .context("required guest artifact")?;
    let wasm = std::fs::read(path).context("read required guest artifact")?;
    imports::verify(&wasm)?;
    journeys::run(&wasm).await
}
