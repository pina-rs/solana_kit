//! Native `solana-kit-publisher` command line entry point.

mod adapters;
mod execute;

use solana_kit_dapp_publisher::cli::{
    DOCS_URL, Flags, NewVersionOptions, PortalTargets, ResumeOptions, enforce_self_update_policy,
    new_version_options, parse_flags, resolve_api_key, resolve_portal_targets, resume_options,
    validate_new_version, validate_resume,
};
use std::{
    collections::BTreeMap,
    io::{IsTerminal, Read},
};

/// Version of this executable's own Cargo package.
///
/// The binary and the `solana_kit_dapp_publisher` library are separate packages
/// that can be released independently, so the executable must not reuse the
/// library's `CLI_VERSION` constant.
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    std::process::exit(run(std::env::args().skip(1).collect()));
}

fn run(arguments: Vec<String>) -> i32 {
    match execute(&arguments) {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("Error: {message}");

            1
        }
    }
}

/// Dispatches help, version and one subcommand without implicit network work.
fn execute(arguments: &[String]) -> Result<(), String> {
    if arguments.is_empty() {
        print_help();

        return Ok(());
    }

    if arguments.iter().any(|argument| argument == "--version") {
        println!("solana-kit-publisher {PACKAGE_VERSION}");

        return Ok(());
    }

    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        print_help();

        return Ok(());
    }

    let resume = arguments.first().is_some_and(|value| value == "resume");
    let flags = parse_flags(&arguments[usize::from(resume)..])
        .map_err(|error| error.message().to_owned())?;
    let environment = environment();

    if resume {
        run_resume_command(&flags, &environment)
    } else {
        run_new_command(&flags, &environment)
    }
}

fn run_new_command(flags: &Flags, environment: &BTreeMap<String, String>) -> Result<(), String> {
    let options = new_version_options(flags);

    validate_new_version(&options, execute::path_exists)
        .map_err(|error| error.message().to_owned())?;
    enforce_self_update_policy(options.skip_self_update, options.local_dev)
        .map_err(|error| error.message().to_owned())?;

    run_new(&options, environment)
}

fn run_resume_command(flags: &Flags, environment: &BTreeMap<String, String>) -> Result<(), String> {
    let options = resume_options(flags);

    validate_resume(&options).map_err(|error| error.message().to_owned())?;
    enforce_self_update_policy(options.skip_self_update, options.local_dev)
        .map_err(|error| error.message().to_owned())?;

    run_resume(&options, environment)
}

fn run_new(
    options: &NewVersionOptions,
    environment: &BTreeMap<String, String>,
) -> Result<(), String> {
    let targets = resolve_targets(
        options.portal_url.as_deref(),
        options.local_dev,
        environment,
    )?;
    let api_key = api_key(
        options.api_key_stdin,
        options.api_key_env.as_deref(),
        environment,
    )?;

    print_targets(options.verbose, &targets);

    let summary = execute::new_publication(options, &targets, &api_key)?;

    for line in summary.lines() {
        println!("{line}");
    }

    Ok(())
}

fn run_resume(
    options: &ResumeOptions,
    environment: &BTreeMap<String, String>,
) -> Result<(), String> {
    let targets = resolve_targets(
        options.portal_url.as_deref(),
        options.local_dev,
        environment,
    )?;
    let api_key = api_key(
        options.api_key_stdin,
        options.api_key_env.as_deref(),
        environment,
    )?;

    print_targets(options.verbose, &targets);

    let summary = execute::resume_publication(options, &targets, &api_key)?;

    for line in summary.lines() {
        println!("{line}");
    }

    Ok(())
}

fn resolve_targets(
    portal_url: Option<&str>,
    local_dev: bool,
    environment: &BTreeMap<String, String>,
) -> Result<PortalTargets, String> {
    resolve_portal_targets(portal_url, local_dev, environment)
        .map_err(|error| error.message().to_owned())
}

fn print_targets(verbose: bool, targets: &PortalTargets) {
    if !verbose {
        return;
    }

    println!("Portal: {}", targets.portal_url);
    println!("Portal API: {}", targets.api_base_url);
}

fn api_key(
    stdin: bool,
    environment_name: Option<&str>,
    environment: &BTreeMap<String, String>,
) -> Result<String, String> {
    let stdin_is_tty = std::io::stdin().is_terminal();
    let mut bytes = Vec::new();

    if stdin && !stdin_is_tty {
        std::io::stdin()
            .read_to_end(&mut bytes)
            .map_err(|_| "failed to read the API key from stdin".to_owned())?;
    }

    resolve_api_key(
        stdin,
        environment_name,
        environment,
        stdin.then_some(bytes.as_slice()),
        stdin_is_tty,
    )
    .map_err(|error| error.message().to_owned())
}

fn environment() -> BTreeMap<String, String> {
    std::env::vars().collect()
}

fn print_help() {
    println!(
        "solana-kit-publisher {PACKAGE_VERSION}

Publish a Solana dApp Store release through the approved portal.

USAGE:
    solana-kit-publisher --apk-file <PATH> --whats-new <TEXT> --keypair <PATH> [options]
    solana-kit-publisher --apk-url <HTTPS_URL> --whats-new <TEXT> --keypair <PATH> [options]
    solana-kit-publisher resume --release-id <ID> --keypair <PATH> [options]
    solana-kit-publisher resume --session-id <ID> --keypair <PATH> [options]

OPTIONS:
    --portal-url <URL>        Portal base URL (default: production or local-dev)
    --api-key-env <NAME>      Environment variable containing the portal API key
    --api-key-stdin           Read the portal API key from piped stdin
    --keypair <PATH>          Solana CLI keypair JSON
    --rpc-url <URL>           Funding preflight RPC URL
    --local-dev               Allow localhost portal endpoints
    --skip-self-update        Only valid with --local-dev
    --idempotency-key <KEY>   Explicit ingestion idempotency key
    --release-id <ID>         Resume by release identity
    --session-id <ID>         Resume by publication session identity
    --resume-release <ID>     Alias of --release-id
    --resume-session <ID>     Alias of --session-id
    --verbose                 Print portal targets
    --version                 Print the CLI version
    --help                    Print this help

The API key defaults to the DAPP_STORE_API_KEY environment variable.
Docs: {DOCS_URL}"
    );
}
