//! Configuration defaults, port validation and real sequential deployment requests.

use super::{fixture::poll_ready, require};
use serde_json::Value;
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::Address;
use solana_kit_surfpool::{
    Surfnet,
    config::{BlockProductionMode, ConfigError, Ports, SurfnetConfig},
    deploy::DeployOptions,
};
use std::{
    cell::RefCell,
    format,
    future::{Ready, ready},
    num::{NonZeroU16, NonZeroUsize},
    rc::Rc,
    vec::Vec,
};

const RPC_PORT: u16 = 10001;
const WS_PORT: u16 = 10002;
const STUDIO_PORT: u16 = 10003;

struct Sequential {
    requests: Rc<RefCell<Vec<Value>>>,
    fail_at: Option<usize>,
}

impl Transport for Sequential {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let Ok(frame) = serde_json::from_slice::<Value>(&request.body) else {
            return ready(Err(TransportError::new(
                solana_kit_client::TransportFailureKind::InvalidResponse,
            )));
        };
        let index = self.requests.borrow().len() + 1;
        let payload = if self.fail_at == Some(index) {
            r#""error":{"code":-32000,"message":"chunk rejected"}"#
        } else {
            r#""result":null"#
        };

        let body = format!(r#"{{"jsonrpc":"2.0","id":{},{} }}"#, frame["id"], payload);
        self.requests.borrow_mut().push(frame);
        ready(Ok(Reply {
            status: 200,
            body: body.into_bytes(),
        }))
    }
}

pub fn run() -> Result<(), &'static str> {
    let address = Address::from_bytes([0; 32]);
    let mut config = SurfnetConfig::default();
    require(
        config.offline
            && config.airdrop_lamports() == 10_000_000_000
            && config.slot_time_ms.get() == 1
            && config.block_production_mode == BlockProductionMode::Transaction,
        "Surfpool startup defaults changed",
    )?;
    let ports = ports()?;
    let args = config
        .start_args(ports, address)
        .map_err(|_| "valid startup ports rejected")?;
    require(
        args.contains(&"--offline".into())
            && args
                .windows(2)
                .any(|pair| pair == ["--studio-port", "10003"]),
        "startup omitted offline or Studio port",
    )?;
    config.remote_rpc_url = Some("https://rpc.invalid/?token=private".into());
    require(
        !format!("{config:?}").contains("private"),
        "startup config debug exposed upstream credentials",
    )?;
    let args = config
        .start_args(ports, address)
        .map_err(|_| "forked startup arguments failed")?;

    require(
        !args.contains(&"--offline".into())
            && args
                .windows(2)
                .any(|pair| pair == ["--rpc-url", "https://rpc.invalid/?token=private"]),
        "remote RPC did not override offline default",
    )?;
    config.rpc_port = Some(ports.websocket);
    require(
        config.start_args(ports, address) == Err(ConfigError::PinnedPortChanged),
        "caller-pinned port changed silently",
    )?;

    deployment(address)?;
    deployment_failure(address)
}

fn ports() -> Result<Ports, &'static str> {
    let rpc = NonZeroU16::new(RPC_PORT).ok_or("invalid RPC fixture port")?;
    let websocket = NonZeroU16::new(WS_PORT).ok_or("invalid WS fixture port")?;
    let studio = NonZeroU16::new(STUDIO_PORT).ok_or("invalid Studio fixture port")?;
    require(
        Ports::new(rpc, rpc, studio) == Err(ConfigError::PortCollision),
        "port collision accepted",
    )?;
    Ports::new(rpc, websocket, studio).map_err(|_| "distinct ports rejected")
}

fn deployment(address: Address) -> Result<(), &'static str> {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let surfnet = Surfnet::new(Client::new(
        Sequential {
            requests: Rc::clone(&requests),
            fail_at: None,
        },
        Limits::default(),
    ));
    let mut options = DeployOptions::new(address, &[1, 2, 3, 4, 255]);
    options.chunk_bytes = NonZeroUsize::new(2).ok_or("invalid chunk fixture size")?;
    options.idl = Some(serde_json::Map::new());
    require(
        poll_ready(surfnet.deploy(options))?.map_err(|_| "byte deployment failed")? == address,
        "deployment returned another program",
    )?;
    let requests = requests.borrow();
    require(
        requests.len() == 4,
        "deployment did not write three chunks then one IDL",
    )?;

    require(
        requests[0]["params"][1] == "0102"
            && requests[1]["params"][2] == 2
            && requests[2]["params"][1] == "ff"
            && requests[2]["params"][2] == 4,
        "program chunk order, offset, or hex changed",
    )?;
    require(
        requests[3]["method"] == "surfnet_registerIdl"
            && requests[3]["params"][0]["address"] == address.to_string(),
        "deployment did not register the target program IDL",
    )
}

fn deployment_failure(address: Address) -> Result<(), &'static str> {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let surfnet = Surfnet::new(Client::new(
        Sequential {
            requests: Rc::clone(&requests),
            fail_at: Some(2),
        },
        Limits::default(),
    ));
    let mut options = DeployOptions::new(address, &[1, 2, 3, 4, 5, 6]);
    options.chunk_bytes = NonZeroUsize::new(2).ok_or("invalid failure fixture chunk size")?;
    options.idl = Some(serde_json::Map::new());
    let failure = poll_ready(surfnet.deploy(options))?;
    require(
        matches!(
            failure,
            Err(solana_kit_surfpool::deploy::DeployError::Rpc(_))
        ),
        "failed chunk did not preserve RPC failure",
    )?;
    require(
        requests.borrow().len() == 2,
        "deployment retried or continued after a failed chunk",
    )
}
