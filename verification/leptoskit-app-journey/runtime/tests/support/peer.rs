//! A bounded loopback protocol peer supplies reads and records every method.
//! Simulated acknowledgements require canonical independently verified wire.
//! This peer is not validator or installed application execution evidence.
use super::transactions::Transactions;
use serde_json::{Value, json};
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

const BYTES: usize = 8192;
const WAIT: Duration = Duration::from_secs(30);
const REQUESTS: usize = 8;

#[derive(Default)]
struct State {
    requests: Mutex<Vec<Value>>,
    transactions: Transactions,
    quiet: AtomicBool,
    reject_reads: AtomicBool,
    cancelled: AtomicBool,
    stopped: AtomicBool,
}

impl State {
    fn record(&self, request: &Value) -> io::Result<()> {
        request["method"]
            .as_str()
            .ok_or_else(|| io::Error::other("RPC method missing"))?;
        let mut requests = self.requests.lock().unwrap();

        if requests.len() == REQUESTS {
            return Err(io::Error::other("RPC request budget"));
        }

        requests.push(request.clone());
        Ok(())
    }
}

pub(super) struct Peer {
    port: u16,
    state: Arc<State>,
    worker: Option<JoinHandle<io::Result<()>>>,
}

impl Peer {
    pub(super) fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let state = Arc::new(State::default());
        let shared = Arc::clone(&state);
        let owner = thread::current();
        let worker = thread::spawn(move || serve(listener, shared, owner));
        Self {
            port,
            state,
            worker: Some(worker),
        }
    }

    pub(super) fn port(&self) -> u16 {
        self.port
    }

    pub(super) fn methods(&self) -> Vec<String> {
        self.requests()
            .iter()
            .map(|request| request["method"].as_str().unwrap().to_owned())
            .collect()
    }

    pub(super) fn requests(&self) -> Vec<Value> {
        self.state.requests.lock().unwrap().clone()
    }

    pub(super) fn allow_transfer(&self, account: &str) {
        self.state.transactions.allow(account);
    }

    pub(super) fn signature(&self) -> Option<String> {
        self.state.transactions.signature()
    }

    pub(super) fn reject_reads(&self, reject: bool) {
        self.state.reject_reads.store(reject, Ordering::Release);
    }

    pub(super) fn withhold(&self) {
        self.state.quiet.store(true, Ordering::Release);
    }

    pub(super) fn cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }

    pub(super) fn close(&mut self) -> io::Result<()> {
        self.state.stopped.store(true, Ordering::Release);
        let Some(worker) = self.worker.take() else {
            return Ok(());
        };
        let wake = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, self.port));
        let result = worker
            .join()
            .map_err(|_| io::Error::other("RPC peer panicked"))?;
        result.and(wake.map(drop))
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        match (thread::panicking(), self.close()) {
            (_, Ok(())) => {}
            (true, Err(error)) => eprintln!("RPC peer cleanup failed: {error}"),
            (false, Err(error)) => panic!("RPC peer cleanup failed: {error}"),
        }
    }
}

/// Keep a quiet request open until actual transport cancellation produces EOF.
/// A recorded request alone cannot establish that the native actor terminated.
fn serve(listener: TcpListener, state: Arc<State>, owner: thread::Thread) -> io::Result<()> {
    loop {
        let (mut stream, _) = listener.accept()?;

        if state.stopped.load(Ordering::Acquire) {
            return Ok(());
        }

        stream.set_read_timeout(Some(WAIT))?;
        stream.set_write_timeout(Some(WAIT))?;
        let request = request(&mut stream)?;
        state.record(&request)?;
        owner.unpark();

        if state.quiet.load(Ordering::Acquire) {
            let mut byte = [0];

            if stream.read(&mut byte)? != 0 {
                return Err(io::Error::other("unexpected pending bytes"));
            }

            state.cancelled.store(true, Ordering::Release);
            owner.unpark();
            continue;
        }

        reply(&mut stream, &request, &state)?;
    }
}

/// Require the actual SDK's finite Content-Length POST. Bound headers and body
/// independently; this fixture never accepts chunked or persistent requests.
fn request(stream: &mut TcpStream) -> io::Result<Value> {
    let mut reader = BufReader::new(stream);
    let mut headers = String::new();
    let mut length = None;

    loop {
        let previous = headers.len();
        let read = reader
            .by_ref()
            .take((BYTES - previous) as u64)
            .read_line(&mut headers)?;

        if read == 0 || headers.len() > BYTES {
            return Err(io::Error::other("RPC header budget"));
        }

        let line = &headers[previous..];

        if line == "\r\n" {
            break;
        }

        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = Some(value.trim().parse::<usize>().map_err(io::Error::other)?);
        }
    }

    let length = length
        .filter(|value| *value <= BYTES)
        .ok_or_else(|| io::Error::other("RPC body budget"))?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    serde_json::from_slice(&body).map_err(io::Error::other)
}

fn response(request: &Value, state: &State) -> io::Result<Value> {
    let id = request["id"]
        .as_u64()
        .ok_or_else(|| io::Error::other("RPC id missing"))?;

    if state.reject_reads.load(Ordering::Acquire) && request["method"] == "getBalance" {
        return Ok(json!({"jsonrpc":"2.0","id":id,"error":{
            "code":-32000,"message":"explicit protocol fixture rejection"
        }}));
    }

    let transactions = &state.transactions;

    let value = match request["method"].as_str() {
        Some("getBalance") => json!({"context":{"slot":10},"value":transactions.balance()}),
        Some("getLatestBlockhash") => json!({"context":{"slot":10},"value":{
            "blockhash":"11111111111111111111111111111111","lastValidBlockHeight":1000}}),
        Some("sendTransaction") => transactions.submit(request)?,
        Some("getSignatureStatuses") => transactions.statuses(request)?,
        _ => return Err(io::Error::other("unexpected RPC method")),
    };

    Ok(json!({"jsonrpc":"2.0","id":id,"result":value}))
}

fn reply(stream: &mut TcpStream, request: &Value, state: &State) -> io::Result<()> {
    let body = serde_json::to_vec(&response(request, state)?)?;
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(&body)
}
