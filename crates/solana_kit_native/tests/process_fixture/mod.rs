//! Child-only loopback RPC fixture, entered through this integration test binary.

use serde_json::Value;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    time::Duration,
};

mod replies;

const REQUEST_BYTES: usize = 16 * 1024;
const REQUESTS: usize = 512;
const FAILURE_EXIT: i32 = 23;

pub fn run() -> i32 {
    match run_child() {
        Ok(()) => 0,
        Err(_) => FAILURE_EXIT,
    }
}

fn run_child() -> io::Result<()> {
    let mode = env::var("SOLANA_KIT_PROCESS_MODE").unwrap();

    match mode.as_str() {
        "early-success" => return Ok(()),
        "early-failure" => return Err(io::Error::other("scripted early exit")),
        "environment" => environment()?,
        "flood" => flood()?,
        _ => {}
    }

    let host: IpAddr = env::var("SURFPOOL_NETWORK_HOST")
        .unwrap_or_else(|_| argument("--host"))
        .parse()
        .unwrap();
    let rpc = TcpListener::bind(SocketAddr::new(host, argument("--port").parse().unwrap()))?;
    let _ws = TcpListener::bind(SocketAddr::new(
        host,
        argument("--ws-port").parse().unwrap(),
    ))?;
    // Stock Studio failure does not establish failure of otherwise ready RPC.
    let _studio = TcpListener::bind(SocketAddr::new(
        host,
        argument("--studio-port").parse().unwrap(),
    ))
    .ok();
    let mut events = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("fixture-calls")?;
    let mut state = replies::State::new(mode, argument("--genesis-hash"));

    serve(rpc, &mut state, &mut events)
}

fn serve(rpc: TcpListener, state: &mut replies::State, events: &mut File) -> io::Result<()> {
    for accepted in rpc.incoming().take(REQUESTS) {
        if exchange(accepted?, state, events)? {
            return Ok(());
        }
    }

    Err(io::Error::other("fixture request budget exhausted"))
}

fn exchange(
    mut socket: TcpStream,
    state: &mut replies::State,
    events: &mut File,
) -> io::Result<bool> {
    let request = request(&mut socket)?;
    let method = request["method"].as_str().unwrap();
    event(events, method)?;
    let (result, exit) = state.reply(method);

    if state.stalled(method) {
        loop {
            std::thread::park();
        }
    }

    let body = serde_json::to_vec(&state.envelope(&request, result)).map_err(io::Error::other)?;
    write!(
        socket,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    socket.write_all(&body)?;
    socket.flush()?;

    Ok(exit)
}

fn argument(name: &str) -> String {
    let args: Vec<_> = env::args().collect();
    let index = args.iter().position(|argument| argument == name).unwrap();

    args[index + 1].clone()
}

fn environment() -> io::Result<()> {
    let isolated = env::current_dir()?;
    // Match the stock CLI's SNAP_REAL_HOME-before-platform-home selection.
    let home = PathBuf::from(
        env::var_os("SNAP_REAL_HOME").unwrap_or_else(|| env::var_os("HOME").unwrap()),
    );
    let selected = env::var("SOLANA_KIT_SELECTED_ENV").unwrap();
    let custom = env::var("SURFPOOL_CALLER_VALUE").unwrap();
    let cleared = env::var_os("SOLANA_KIT_COMMAND_ENV").is_none()
        && env::var_os("CARGO_MANIFEST_DIR").is_none();

    if home != isolated || selected != "selected" || custom != "preserved" || !cleared {
        return Err(io::Error::other("explicit environment policy failed"));
    }

    fixed_paths(&isolated)?;

    configuration_environment_is_absent()
}

fn fixed_paths(isolated: &std::path::Path) -> io::Result<()> {
    for (name, suffix) in [
        ("HOME", ""),
        ("USERPROFILE", ""),
        ("XDG_CONFIG_HOME", "config"),
        ("TMPDIR", "tmp"),
        ("TMP", "tmp"),
        ("TEMP", "tmp"),
    ] {
        if env::var_os(name).unwrap() != isolated.join(suffix) {
            return Err(io::Error::other("explicit runtime paths were overridden"));
        }
    }

    Ok(())
}

fn configuration_environment_is_absent() -> io::Result<()> {
    for name in [
        "SURFPOOL_NETWORK_HOST",
        "SURFPOOL_STUDIO_HOST",
        "SURFPOOL_PUBLIC_HOST",
        "SURFPOOL_PUBLIC_RPC_URL",
        "SURFPOOL_PUBLIC_WS_URL",
        "SURFPOOL_PUBLIC_STUDIO_URL",
        "SURFPOOL_DATASOURCE_RPC_URL",
        "SURFPOOL_METRICS_ENABLED",
        "SURFPOOL_METRICS_ADDR",
    ] {
        if env::var_os(name).is_some() {
            return Err(io::Error::other("stock configuration environment survived"));
        }
    }

    Ok(())
}

fn flood() -> io::Result<()> {
    let block = [b'x'; 8192];

    for _ in 0..128 {
        io::stdout().write_all(&block)?;
        io::stderr().write_all(&block)?;
    }

    Ok(())
}

fn event(events: &mut File, method: &str) -> io::Result<()> {
    writeln!(events, "{method}")?;
    events.flush()
}

fn request(socket: &mut TcpStream) -> io::Result<Value> {
    socket.set_read_timeout(Some(Duration::from_secs(3)))?;
    socket.set_write_timeout(Some(Duration::from_secs(3)))?;
    let mut bytes = Vec::new();

    while !bytes.ends_with(b"\r\n\r\n") {
        if bytes.len() >= REQUEST_BYTES {
            return Err(io::Error::other("fixture headers too large"));
        }

        let mut byte = [0];
        socket.read_exact(&mut byte)?;
        bytes.push(byte[0]);
    }

    let header = std::str::from_utf8(&bytes).unwrap();
    let length: usize = header
        .lines()
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();

    if length > REQUEST_BYTES {
        return Err(io::Error::other("fixture body too large"));
    }

    let mut body = vec![0; length];
    socket.read_exact(&mut body)?;
    serde_json::from_slice(&body).map_err(io::Error::other)
}
