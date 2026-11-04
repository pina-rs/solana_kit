//! Reserved numeric loopback sockets and bounded validator port windows.

use crate::node::NodeError;
use std::net::{Ipv4Addr, TcpListener};

pub(crate) const DYNAMIC_PORTS: u16 = 200;
const MIN_PORT: u16 = 10_000;
const WINDOW_CANDIDATES: usize = 250;
const PORT_ATTEMPTS: usize = 64;

pub(crate) struct Ports {
    pub(crate) rpc: u16,
    pub(crate) faucet: u16,
    pub(crate) dynamic_base: u16,
    listeners: Option<Listeners>,
}

struct Listeners {
    // Held only to reserve the ports until the child is spawned.
    _rpc: TcpListener,
    _rpc_ws: TcpListener,
    _faucet: TcpListener,
}

impl Ports {
    pub(crate) fn reserve(nonce: &[u8; 2]) -> Result<Self, NodeError> {
        let (rpc_listener, rpc_ws_listener, rpc) = reserve_rpc_pair()?;
        let (faucet_listener, faucet) = reserve_port()?;
        let dynamic_base = dynamic_base(nonce, &[rpc, rpc.saturating_add(1), faucet]);

        Ok(Self {
            rpc,
            faucet,
            dynamic_base,
            listeners: Some(Listeners {
                _rpc: rpc_listener,
                _rpc_ws: rpc_ws_listener,
                _faucet: faucet_listener,
            }),
        })
    }

    pub(crate) fn release(&mut self) {
        self.listeners = None;
    }
}

fn reserve_port() -> Result<(TcpListener, u16), NodeError> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let port = listener.local_addr()?.port();

    Ok((listener, port))
}

fn reserve_rpc_pair() -> Result<(TcpListener, TcpListener, u16), NodeError> {
    for _ in 0..PORT_ATTEMPTS {
        let first = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        let port = first.local_addr()?.port();
        let Some(next) = port.checked_add(1) else {
            continue;
        };

        if let Ok(second) = TcpListener::bind((Ipv4Addr::LOCALHOST, next)) {
            return Ok((first, second, port));
        }
    }

    Err(NodeError::Readiness(
        "no adjacent free loopback RPC ports".to_owned(),
    ))
}

fn dynamic_base(nonce: &[u8; 2], reserved: &[u16]) -> u16 {
    let start = usize::from(u16::from_le_bytes(*nonce) % WINDOW_CANDIDATES as u16);

    for step in 0..WINDOW_CANDIDATES {
        let index = (start + step) % WINDOW_CANDIDATES;
        let base = MIN_PORT + index as u16 * DYNAMIC_PORTS;
        let end = base + DYNAMIC_PORTS;

        if reserved.iter().all(|port| *port < base || *port >= end) {
            return base;
        }
    }

    MIN_PORT
}
