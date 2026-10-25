//! Nonblocking WASI resources adapted to the established TLS/WebSocket codecs.

use crate::{Reactor, SocketError};
use std::io::{self, Read, Write};
use wasip2::io::{
    poll::Pollable,
    streams::{InputStream, OutputStream, StreamError},
};
use wasip2::sockets::{instance_network::instance_network, network::*, tcp::TcpSocket};

const IO_CHUNK_BYTES: usize = 65_536;

pub(crate) struct Tcp {
    input: InputStream,
    output: OutputStream,
    socket: TcpSocket,
    writing: bool,
    flushing: bool,
}

impl Tcp {
    pub async fn connect(
        host: &str,
        port: u16,
        reactor: &Reactor,
        deadline: u64,
    ) -> Result<Self, SocketError> {
        let network = instance_network();
        let resolver = wasip2::sockets::ip_name_lookup::resolve_addresses(&network, host)
            .map_err(SocketError::Host)?;
        let mut last_error = SocketError::NoAddress;

        loop {
            let address = match resolver.resolve_next_address() {
                Ok(Some(address)) => address,
                Ok(None) => return Err(last_error),
                Err(ErrorCode::WouldBlock) => {
                    wait(reactor, resolver.subscribe(), deadline).await?;
                    continue;
                }
                Err(error) => return Err(SocketError::Host(error)),
            };

            match Self::connect_address(&network, address, port, reactor, deadline).await {
                Ok(tcp) => return Ok(tcp),
                Err(error @ SocketError::Host(_)) => last_error = error,
                Err(error) => return Err(error),
            }
        }
    }

    async fn connect_address(
        network: &Network,
        address: IpAddress,
        port: u16,
        reactor: &Reactor,
        deadline: u64,
    ) -> Result<Self, SocketError> {
        let (family, address) = address_with_port(address, port);
        let socket = wasip2::sockets::tcp_create_socket::create_tcp_socket(family)
            .map_err(SocketError::Host)?;
        socket
            .start_connect(network, address)
            .map_err(SocketError::Host)?;

        let (input, output) = loop {
            match socket.finish_connect() {
                Ok(streams) => break streams,
                Err(ErrorCode::WouldBlock) => wait(reactor, socket.subscribe(), deadline).await?,
                Err(error) => return Err(SocketError::Host(error)),
            }
        };

        Ok(Self {
            input,
            output,
            socket,
            writing: false,
            flushing: false,
        })
    }

    pub fn signal(&self) -> Pollable {
        if self.writing {
            self.output.subscribe()
        } else {
            self.input.subscribe()
        }
    }
}

impl Drop for Tcp {
    fn drop(&mut self) {
        // Resource destruction closes the descriptor even when shutdown reports
        // an already-closed peer. Streams drop before their parent socket.
        let _ = self
            .socket
            .shutdown(wasip2::sockets::tcp::ShutdownType::Both);
    }
}

impl Read for Tcp {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }

        match self.input.read(bytes.len().min(IO_CHUNK_BYTES) as u64) {
            Ok(chunk) if chunk.is_empty() => {
                self.writing = false;
                Err(io::ErrorKind::WouldBlock.into())
            }

            Ok(chunk) => {
                bytes[..chunk.len()].copy_from_slice(&chunk);
                Ok(chunk.len())
            }
            Err(StreamError::Closed) => Ok(0),
            Err(error) => Err(stream_error(error)),
        }
    }
}

impl Write for Tcp {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }

        let allowed = self.output.check_write().map_err(stream_error)?;

        if allowed == 0 {
            self.writing = true;

            return Err(io::ErrorKind::WouldBlock.into());
        }

        let count = bytes.len().min(allowed.min(IO_CHUNK_BYTES as u64) as usize);
        self.output.write(&bytes[..count]).map_err(stream_error)?;
        self.output.flush().map_err(stream_error)?;
        self.flushing = true;

        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.flushing {
            self.output.flush().map_err(stream_error)?;
            self.flushing = true;
        }

        if self.output.check_write().map_err(stream_error)? == 0 {
            self.writing = true;

            return Err(io::ErrorKind::WouldBlock.into());
        }

        self.flushing = false;

        Ok(())
    }
}

fn stream_error(error: StreamError) -> io::Error {
    match error {
        StreamError::Closed => io::ErrorKind::BrokenPipe.into(),
        StreamError::LastOperationFailed(error) => io::Error::other(error.to_debug_string()),
    }
}

fn address_with_port(address: IpAddress, port: u16) -> (IpAddressFamily, IpSocketAddress) {
    match address {
        IpAddress::Ipv4(address) => (
            IpAddressFamily::Ipv4,
            IpSocketAddress::Ipv4(Ipv4SocketAddress { address, port }),
        ),
        IpAddress::Ipv6(address) => (
            IpAddressFamily::Ipv6,
            IpSocketAddress::Ipv6(Ipv6SocketAddress {
                address,
                port,
                flow_info: 0,
                scope_id: 0,
            }),
        ),
    }
}

pub(crate) async fn wait(
    reactor: &Reactor,
    signal: Pollable,
    deadline: u64,
) -> Result<(), SocketError> {
    reactor
        .wait(signal, deadline)
        .map_err(|()| SocketError::ReadinessLimit)?
        .await
        .map_err(|()| SocketError::Deadline)
}

pub(crate) enum Stream {
    Plain(Tcp),
    Tls(Box<rustls::StreamOwned<rustls::ClientConnection, Tcp>>),
}

impl Stream {
    pub fn signal(&self) -> Pollable {
        match self {
            Self::Plain(tcp) => tcp.signal(),
            Self::Tls(tls) => tls.sock.signal(),
        }
    }
}

impl Read for Stream {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Plain(tcp) => tcp.read(bytes),
            Self::Tls(tls) => tls.read(bytes),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        match self {
            Self::Plain(tcp) => tcp.write(bytes),
            Self::Tls(tls) => tls.write(bytes),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Plain(tcp) => tcp.flush(),
            Self::Tls(tls) => tls.flush(),
        }
    }
}
