//! Keep adjacent RPC, socket and faucet authorities exclusive between journeys.
use crate::{Error, Result};
use std::{
    fs,
    net::{Ipv4Addr, TcpListener},
    path::{Path, PathBuf},
};

const RPC_BASE: u16 = 18000;
const DYNAMIC_BASE: u16 = 20000;
const WIDTH: u16 = 80;
const LEASES: u16 = 400;
const PORT_STRIDE: u16 = 4;
const LISTENERS: usize = 3;

pub(crate) struct Ports {
    pub rpc: u16,
    pub faucet: u16,
    pub gossip: u16,
    pub dynamic: String,
    directory: PathBuf,
    held: Vec<TcpListener>,
}

impl Ports {
    pub(crate) fn acquire() -> Result<Self> {
        let root = std::env::temp_dir().join("solana-kit-installed-node-ports");
        fs::create_dir_all(&root)?;

        for index in 0..LEASES {
            if let Some(ports) = Self::reserve(&root, index)? {
                return Ok(ports);
            }
        }

        Err(Error::Configuration("no exclusive local node ports".into()))
    }

    fn reserve(root: &Path, index: u16) -> Result<Option<Self>> {
        let rpc = RPC_BASE + index * PORT_STRIDE;
        let directory = root.join(rpc.to_string());

        match fs::create_dir(&directory) {
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => return Ok(None),
            result => result?,
        }

        let gossip = DYNAMIC_BASE + index * WIDTH;
        let mut ports = Self {
            rpc,
            faucet: rpc + 2,
            gossip,
            dynamic: format!("{}-{}", gossip + 1, gossip + WIDTH),
            directory,
            held: Vec::new(),
        };
        let available = ports.hold()?;
        Ok(available.then_some(ports))
    }

    fn hold(&mut self) -> Result<bool> {
        for port in [self.rpc, self.rpc + 1, self.faucet] {
            let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port));

            if listener
                .as_ref()
                .is_err_and(|error| error.kind() == std::io::ErrorKind::AddrInUse)
            {
                return Ok(false);
            }

            self.held.push(listener?);
        }

        Ok(self.held.len() == LISTENERS)
    }

    pub(crate) fn release_listeners(&mut self) {
        self.held.clear();
    }
}

impl Drop for Ports {
    fn drop(&mut self) {
        self.held.clear();

        if let Err(error) = fs::remove_dir(&self.directory) {
            eprintln!("Could not release owned node port lease: {error}");
        }
    }
}
