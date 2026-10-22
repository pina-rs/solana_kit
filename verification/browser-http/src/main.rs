//! Rust-owned loopback fixture and real Chrome verification command.
mod graphs;
mod server;
use std::{
    net::TcpListener,
    process::{Command, ExitCode},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1).as_deref() == Some("--graphs") {
        return graphs::run();
    }

    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let address = listener.local_addr()?;
    let stop = Arc::new(AtomicBool::new(false));
    let events = server::Events::default();
    let sink_listener = TcpListener::bind("127.0.0.1:0")?;
    sink_listener.set_nonblocking(true)?;
    let sink_url = format!("http://{}/sink", sink_listener.local_addr()?);

    let sink_events = events.clone();
    let sink_stop = stop.clone();
    let sink = thread::spawn(move || sink_server(sink_listener, sink_stop, sink_events));
    let errors = Arc::new(Mutex::new(Vec::new()));
    let stop_server = stop.clone();
    let errors_server = errors.clone();

    let server =
        thread::spawn(move || run_server(listener, stop_server, events, errors_server, sink_url));
    let result = chrome(&format!("http://{address}"));
    stop.store(true, Ordering::SeqCst);
    server.join().expect("fixture server panicked");
    sink.join().expect("fixture sink panicked");
    let result = result?;
    let errors = errors.lock().unwrap();

    if !result.success() || !errors.is_empty() {
        return Err(format!("Chrome result {result}; fixture failures {errors:?}").into());
    }

    println!("Real Chrome HTTP journeys passed; all fixture threads joined.");
    Ok(())
}
fn chrome(endpoint: &str) -> std::io::Result<std::process::ExitStatus> {
    Command::new("cargo")
        .args([
            "test",
            "-p",
            "solana_kit_browser_http_verification",
            "--lib",
            "--target",
            "wasm32-unknown-unknown",
            "--locked",
            "--",
            "--nocapture",
        ])
        .env("SOLANA_KIT_BROWSER_FIXTURE", endpoint)
        .env("WASM_BINDGEN_USE_BROWSER", "1")
        .env("WASM_BINDGEN_USE_BROWSER_ONLY", "1")
        .env("WASM_BINDGEN_TEST_TIMEOUT", "60")
        .status()
}
/// Every accepted socket gets one joined owner so a failed browser cannot leak fixtures.
fn run_server(
    listener: TcpListener,
    stop: Arc<AtomicBool>,
    events: server::Events,
    errors: Arc<Mutex<Vec<String>>>,
    sink_url: String,
) {
    let mut tasks = Vec::new();

    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                let events = events.clone();
                let errors = errors.clone();
                let sink_url = sink_url.clone();
                tasks.push(thread::spawn(move || {
                    record(stream, events, errors, sink_url)
                }));
            }

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2))
            }

            Err(error) => {
                errors.lock().unwrap().push(error.to_string());
                break;
            }
        }
    }

    for task in tasks {
        task.join().expect("fixture worker panicked");
    }
}
fn record(
    stream: std::net::TcpStream,
    events: server::Events,
    errors: Arc<Mutex<Vec<String>>>,
    sink_url: String,
) {
    if let Err(error) = server::serve(stream, events, &sink_url) {
        errors.lock().unwrap().push(error.to_string());
    }
}

fn sink_server(listener: TcpListener, stop: Arc<AtomicBool>, events: server::Events) {
    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                events.lock().unwrap().insert("sink".into(), (true, false));
                drop(stream);
            }

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2))
            }

            Err(error) => panic!("redirect sink failed: {error}"),
        }
    }
}

#[cfg(test)]
mod server_tests;
