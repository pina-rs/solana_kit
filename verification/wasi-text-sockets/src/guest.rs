//! Ordinary downstream consumer of the public Preview2 socket API.
use anyhow::{Result, ensure};
use solana_kit_wasi::{Reactor, SocketConfig, SocketError, WasiWebSocket};
use std::{
    future::Future,
    io::Write,
    pin::Pin,
    task::{Context, Waker},
    time::Duration,
};

const OPERATION_SECONDS: u64 = 2;
const FRAME_BYTES: usize = 64;
const MESSAGE_BYTES: usize = 96;
const WRITE_BYTES: usize = 128;
const QUIET_NANOS: u64 = 350_000_000;
const HOLD_NANOS: u64 = 1_000_000_000;

fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let mode = &arguments[1];
    let endpoint = &arguments[2];
    let reactor = Reactor::default();
    let connected = reactor.block_on(WasiWebSocket::connect(
        reactor.clone(),
        endpoint,
        config(mode),
    ))?;

    if let Some(socket) = connection_failure(mode, connected)? {
        journey(mode, &reactor, socket)?;
    }

    ensure!(reactor.registrations() == 0, "retained registration");

    println!("PASS {mode}");
    Ok(())
}

fn config(mode: &str) -> SocketConfig {
    let certificate = if mode == "wrong-ca" {
        include_bytes!("../../wasi-sockets/fixtures/wrong-ca.der").as_slice()
    } else {
        include_bytes!("../../wasi-sockets/fixtures/ca.der").as_slice()
    };

    SocketConfig {
        connect_timeout: Duration::from_secs(OPERATION_SECONDS),
        operation_timeout: Duration::from_secs(OPERATION_SECONDS),
        max_frame_bytes: FRAME_BYTES,
        max_message_bytes: MESSAGE_BYTES,
        max_write_buffer_bytes: WRITE_BYTES,
        root_certificates: vec![certificate.to_vec()],
    }
}

fn connection_failure(
    mode: &str,
    connected: Result<WasiWebSocket, SocketError>,
) -> Result<Option<WasiWebSocket>> {
    let expected = match mode {
        "wrong-ca" | "wrong-name" => matches!(connected, Err(SocketError::Tls(_))),
        "denied" => matches!(connected, Err(SocketError::Host(_))),
        "bad-upgrade" => matches!(connected, Err(SocketError::Protocol)),
        _ => return Ok(Some(connected?)),
    };

    ensure!(expected, "incorrect connect failure for {mode}");
    Ok(None)
}

fn journey(mode: &str, reactor: &Reactor, socket: WasiWebSocket) -> Result<()> {
    match mode {
        "exchange" | "plain" | "corrupt" | "duplex" | "fragment" => exchange(mode, reactor, socket),
        "cancel" | "unload" | "retain" => quiet(mode, reactor, socket),
        _ => rejected_message(mode, reactor, socket),
    }
}

fn exchange(mode: &str, reactor: &Reactor, mut socket: WasiWebSocket) -> Result<()> {
    if mode == "duplex" {
        duplex_send(&mut socket, reactor)?;
    } else {
        reactor.block_on(socket.send("subscribe:7"))??;
    }

    let value = reactor.block_on(socket.receive())??;
    ensure!(
        value.as_deref() == Some("notification:41"),
        "wrong text payload"
    );
    reactor.block_on(socket.close())??;
    Ok(())
}

fn duplex_send(socket: &mut WasiWebSocket, reactor: &Reactor) -> Result<()> {
    let mut context = Context::from_waker(Waker::noop());
    ensure!(
        socket.poll_receive(&mut context).is_pending(),
        "receive not pending"
    );
    ensure!(reactor.registrations() == 1, "receive did not register");
    socket.start_send("subscribe:7")?;
    reactor.block_on(std::future::poll_fn(|cx| socket.poll_flush(cx)))??;
    Ok(())
}

fn quiet(mode: &str, reactor: &Reactor, mut socket: WasiWebSocket) -> Result<()> {
    let mut future = Box::pin(socket.receive());
    ensure!(
        Pin::new(&mut future)
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    ensure!(reactor.registrations() == 1);
    drop(future);
    ensure!(
        reactor.registrations() == 1,
        "cancelled wrapper lost parser wait"
    );
    println!("PENDING {mode}");
    std::io::stdout().flush()?;

    if mode == "unload" {
        hold();
    }

    if mode == "cancel" {
        socket.shutdown();
        ensure!(reactor.registrations() == 0);
    }

    wasip2::clocks::monotonic_clock::subscribe_duration(QUIET_NANOS).block();
    drop(socket);
    Ok(())
}

fn hold() -> ! {
    loop {
        wasip2::clocks::monotonic_clock::subscribe_duration(HOLD_NANOS).block();
    }
}

fn rejected_message(mode: &str, reactor: &Reactor, mut socket: WasiWebSocket) -> Result<()> {
    let received = reactor.block_on(socket.receive())?;

    let expected = match mode {
        "oversized" => matches!(received, Err(SocketError::Limit)),
        "binary" => matches!(received, Err(SocketError::Binary)),
        "invalid-utf8" | "masked" => matches!(received, Err(SocketError::Protocol)),
        "abrupt" => matches!(received, Err(SocketError::AbruptClose)),
        _ => false,
    };

    ensure!(expected, "wrong failure for {mode}: {received:?}");
    Ok(())
}
