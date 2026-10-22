//! Application-owned fixture tasks and process invocation.
use std::{
    net::TcpListener,
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let address = listener.local_addr()?;
    let tls = TcpListener::bind("127.0.0.1:0")?;
    let tls_address = tls.local_addr()?;

    let stop = Arc::new(AtomicBool::new(false));
    let tls_stop = stop.clone();
    let tls_owner = thread::spawn(move || crate::tls::reject(tls, tls_stop));
    let completed = Arc::new(AtomicUsize::new(0));
    let counts = completed.clone();
    let stop_server = stop.clone();
    let fixture = thread::spawn(move || serve(listener, stop_server, counts));

    let result = Command::new("cargo")
        .args([
            "test",
            "-p",
            "solana_kit_browser_websocket_verification",
            "--lib",
            "--target",
            "wasm32-unknown-unknown",
            "--offline",
            "--locked",
            "--",
            "--nocapture",
        ])
        .args(std::env::args().skip(1))
        .env("SOLANA_KIT_WS_FIXTURE", format!("ws://{address}"))
        .env(
            "SOLANA_KIT_WSS_FIXTURE",
            format!("wss://localhost:{}", tls_address.port()),
        )
        .env("WASM_BINDGEN_USE_BROWSER", "1")
        .env("WASM_BINDGEN_USE_BROWSER_ONLY", "1")
        .status();
    stop.store(true, Ordering::SeqCst);
    let accepted = fixture.join().expect("fixture owner panicked");
    tls_owner.join().expect("TLS owner panicked");

    assert_eq!(accepted, completed.load(Ordering::SeqCst));
    assert!(result?.success(), "actual Chrome failed");
    println!("Chrome passed. {accepted} fixture connections completed and all tasks joined.");
    Ok(())
}
// Nonblocking accept lets shutdown stop the listener before joining every worker.
fn serve(listener: TcpListener, stop: Arc<AtomicBool>, completed: Arc<AtomicUsize>) -> usize {
    let mut tasks = Vec::new();
    let handshake = Arc::new(AtomicBool::new(false));

    while !stop.load(Ordering::SeqCst) {
        if let Some(stream) = accept(&listener) {
            let completed = completed.clone();
            let handshake = handshake.clone();
            tasks.push(thread::spawn(move || {
                let result =
                    crate::server::serve(stream, handshake).map_err(|error| error.to_string());
                completed.fetch_add(1, Ordering::SeqCst);
                result
            }));
        }
    }

    let accepted = tasks.len();

    for task in tasks {
        task.join()
            .expect("fixture panicked")
            .expect("fixture failed");
    }

    accepted
}
fn accept(listener: &TcpListener) -> Option<std::net::TcpStream> {
    match listener.accept() {
        Ok((stream, _)) => Some(stream),

        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
            thread::sleep(Duration::from_millis(2));
            None
        }

        Err(error) => panic!("listener failed {error}"),
    }
}
