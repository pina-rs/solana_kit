//! One application owner routes every capability and joins all native work.
use crate::{Host, config::Endpoints, native, startup::Startup, timers::Timers};
use leptoskit_bridge::Error;
use leptoskit_bridge_ffi::app::{AppService, ServiceClose, ServiceWork};
use solana_kit_leptoskit::{
    HostLimits, HttpDriver, Routed, SocketDriver, SocketLimits, wallet_service::WalletService,
};
use solana_kit_native::{HttpConfig, NativeHttp, WsConfig};
use std::{
    future::Future,
    path::Path,
    task::{Context, Poll},
    thread::{self, ThreadId},
    time::Duration,
};
use tokio::runtime::{Builder, Handle, Runtime};

const BODY_BYTES: usize = 64 * 1024;
const TURNS: usize = 32;
const SOCKET_LIFETIME: Duration = Duration::from_secs(600);

pub(super) struct Service {
    http: HttpDriver,
    sockets: SocketDriver,
    timers: Timers,
    wallet: WalletService,
    runtime: Runtime,
    queued: ServiceWork,
    owner: ThreadId,
    startup: Startup,
}

impl Service {
    pub(super) fn install(
        host: &mut Host,
        data: &Path,
        endpoints: Endpoints,
        startup: Startup,
    ) -> Result<Self, Error> {
        // Public generated hosts call the service from their native UI thread.
        // Refuse an ambient runtime before constructing an owner whose close is synchronous.
        if Handle::try_current().is_ok() {
            return Err(Error::InvalidArgument);
        }

        let runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(native)?;
        let http = http(host, endpoints.http, endpoints.quiet)?;
        let sockets = sockets(host, endpoints.socket, endpoints.quiet_socket)?;
        let timers = Timers::new(host)?;
        let wallet = WalletService::install(host, data).map_err(native)?;
        Ok(Self {
            http,
            sockets,
            timers,
            wallet,
            runtime,
            queued: ServiceWork::default(),
            owner: thread::current().id(),
            startup,
        })
    }

    fn completions(&mut self, host: &mut Host, context: &mut Context<'_>) -> Result<bool, Error> {
        let mut completed = false;
        let http = {
            let completion = self.http.next_completion();
            std::pin::pin!(completion).poll(context)
        };

        if let Poll::Ready(completion) = http {
            self.http
                .deliver(host, completion.map_err(native)?)
                .map_err(native)?;
            completed = true;
        }

        let socket = {
            let completion = self.sockets.next_completion();
            std::pin::pin!(completion).poll(context)
        };

        if let Poll::Ready(completion) = socket {
            self.sockets
                .deliver(host, completion.map_err(native)?)
                .map_err(native)?;
            completed = true;
        }

        Ok(self.timers.poll(host, context)? || completed)
    }
}

impl AppService for Service {
    /// A completion can synchronously resume the guest into another capability.
    /// Drain that work through the same owner, retaining and waking the next quantum at the budget.
    fn poll(&mut self, host: &mut Host, context: &mut Context<'_>) -> Result<ServiceWork, Error> {
        if thread::current().id() != self.owner || Handle::try_current().is_ok() {
            return Err(Error::InvalidArgument);
        }

        self.startup
            .observe(
                host.surface().root().is_some(),
                !host.surface().backend().log().is_empty(),
            )
            .map_err(native)?;
        let handle = self.runtime.handle().clone();
        let _entered = handle.enter();

        let mut incoming = std::mem::take(&mut self.queued);
        append(&mut incoming, drain(host));
        let mut platform = ServiceWork::default();

        for _ in 0..TURNS {
            let routed = self
                .http
                .route(
                    host,
                    Routed {
                        requests: incoming.requests,
                        cancelled: incoming.cancelled,
                        guest_woken: false,
                    },
                )
                .map_err(native)?;
            let routed = self.sockets.route(host, routed).map_err(native)?;
            let routed = self.timers.route(host, routed)?;
            let guest_woken = routed.guest_woken;
            let work = self.wallet.route(
                host,
                context,
                ServiceWork {
                    requests: routed.requests,
                    cancelled: routed.cancelled,
                },
            )?;
            append(&mut platform, work);
            let completed = self.completions(host, context)?;
            incoming = drain(host);

            if incoming.requests.is_empty()
                && incoming.cancelled.is_empty()
                && !completed
                && !guest_woken
            {
                return Ok(platform);
            }
        }

        self.queued = incoming;
        context.waker().wake_by_ref();
        Ok(platform)
    }

    fn close(self: Box<Self>) -> ServiceClose {
        let Self {
            mut http,
            mut sockets,
            mut timers,
            wallet,
            runtime,
            ..
        } = *self;
        let mut cleanup = Box::pin(async move {
            let (http, sockets, timers, wallet) = tokio::join!(
                http.shutdown(),
                sockets.shutdown(),
                timers.close(),
                Box::new(wallet).close(),
            );
            http.map_err(native)
                .and(sockets.map_err(native))
                .and(timers)
                .and(wallet)
        });
        let mut runtime = Some(runtime);
        Box::pin(std::future::poll_fn(move |context| {
            let result = {
                let _entered = runtime.as_ref().expect("live close runtime").enter();
                cleanup.as_mut().poll(context)
            };

            if result.is_ready() {
                runtime.take();
            }

            result
        }))
    }
}

fn http(host: &mut Host, endpoint: String, quiet: Option<String>) -> Result<HttpDriver, Error> {
    let mut driver = HttpDriver::new(
        host,
        HostLimits {
            jobs: 8,
            request_bytes: BODY_BYTES,
            reply_bytes: BODY_BYTES,
        },
    )
    .map_err(native)?;

    for (name, granted) in [("solana.rpc", true), ("denied", false)] {
        let service = NativeHttp::new(&endpoint, HttpConfig::default()).map_err(native)?;
        driver
            .install(host, name, service, granted)
            .map_err(native)?;
    }

    if let Some(endpoint) = quiet {
        let service = NativeHttp::new(&endpoint, HttpConfig::default()).map_err(native)?;
        driver
            .install(host, "quiet", service, true)
            .map_err(native)?;
    }

    Ok(driver)
}

fn sockets(
    host: &mut Host,
    endpoint: String,
    quiet: Option<String>,
) -> Result<SocketDriver, Error> {
    let mut driver = SocketDriver::new(
        host,
        SocketLimits {
            sockets: 2,
            messages: 32,
            frame_bytes: BODY_BYTES,
        },
    )
    .map_err(native)?;

    for (name, endpoint) in [("solana.socket", Some(endpoint)), ("quiet.socket", quiet)] {
        let Some(endpoint) = endpoint else {
            continue;
        };
        let config = WsConfig {
            max_frame_bytes: BODY_BYTES,
            max_message_bytes: BODY_BYTES,
            operation_timeout: SOCKET_LIFETIME,
            ..Default::default()
        };
        driver
            .install(host, name, endpoint, config, true)
            .map_err(native)?;
    }

    Ok(driver)
}

fn append(output: &mut ServiceWork, incoming: ServiceWork) {
    output.requests.extend(incoming.requests);
    output.cancelled.extend(incoming.cancelled);
}

fn drain(host: &mut Host) -> ServiceWork {
    ServiceWork {
        cancelled: std::iter::from_fn(|| host.plugins().next_cancelled()).collect(),
        requests: std::iter::from_fn(|| host.plugins().next_request()).collect(),
    }
}
