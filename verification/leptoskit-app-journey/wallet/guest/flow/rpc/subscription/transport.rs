//! One real socket routes typed account and root subscriptions together.
use super::{AppError, Error, Routing, finish, timed};
use solana_kit_client::{
    methods::{Commitment, accounts::AccountEncoding},
    subscriptions::{
        Account, AccountConfig, Budget, Connection, Root, Status, Subscription, SubscriptionMethod,
    },
};
use solana_kit_core::Address;
use solana_kit_leptoskit::{GuestWebSocket, SocketState};
use std::{future::Future, pin::Pin, task::Poll};

pub(super) const ACCOUNT: Address = Address::from_bytes([97; 32]);
// Native operation timers own expiry; no invented guest clock ticks are supplied.
const ACK_DEADLINE: u64 = u64::MAX;
const SOCKET_MESSAGES: usize = 32;
const FRAME_BYTES: usize = 64 * 1024;
const UNRELATED_EVENTS: usize = 64;

pub(super) struct Transport {
    pub owner: Connection,
    pub account: Subscription<Account>,
    pub root: Subscription<Root>,
    pub generation: u64,
    socket: GuestWebSocket,
}

impl Transport {
    pub fn new(generation: u64) -> Result<Self, AppError> {
        let mut owner = Connection::new(generation, Budget::default());
        let account = owner.subscribe(
            Account(
                ACCOUNT,
                Some(AccountConfig {
                    commitment: Some(Commitment::Confirmed),
                    encoding: Some(AccountEncoding::Base64),
                }),
            ),
            ACK_DEADLINE,
        )?;
        let root = owner.subscribe(Root, ACK_DEADLINE)?;
        let socket = GuestWebSocket::new("solana.socket", SOCKET_MESSAGES, FRAME_BYTES)?;
        Ok(Self {
            owner,
            account,
            root,
            generation,
            socket,
        })
    }

    pub async fn start(&mut self, routing: &mut Routing<'_>) -> Result<(), AppError> {
        while self.socket.state() == SocketState::Connecting {
            self.receive(routing).await?;
        }

        self.flush().await?;

        loop {
            let account = active(&mut self.owner, &self.account)?;
            let root = active(&mut self.owner, &self.root)?;

            if account && root {
                return Ok(());
            }

            self.receive(routing).await?;
        }
    }

    /// Borrow the application's sole receiver for one wake and preserve unrelated
    /// work instead of creating a competing event consumer for this socket.
    pub async fn receive(&mut self, routing: &mut Routing<'_>) -> Result<(), AppError> {
        let mut next = routing.events.recv();
        let event = std::future::poll_fn(|context| {
            if let Poll::Ready(result) = self.socket.poll_lease(context) {
                result?;
                return Poll::Ready(Err(leptoskit_bridge::Error::Unavailable.into()));
            }

            Pin::new(&mut next)
                .poll(context)
                .map_err(Into::<AppError>::into)
        })
        .await?;
        drop(next);

        if let Some(event) = self.socket.route(event)? {
            if routing.unrelated.len() == UNRELATED_EVENTS {
                return Err(leptoskit_bridge::Error::Busy.into());
            }

            routing.unrelated.push_back(event);
        }

        while let Some(frame) = self.socket.try_receive()? {
            self.owner.ingest(self.generation, frame.as_bytes())?;
        }

        Ok(())
    }

    async fn flush(&mut self) -> Result<(), AppError> {
        while let Some(frame) = self.owner.outbound() {
            let text = std::str::from_utf8(&frame)?;
            self.socket.send(text)?.await?;
        }

        Ok(())
    }

    pub async fn cancel_account(&mut self, routing: &mut Routing<'_>) -> Result<(), AppError> {
        self.owner.unsubscribe(&self.account, ACK_DEADLINE)?;
        self.flush().await?;

        while !self.owner.settled(&self.account)? {
            self.receive(routing).await?;
        }

        match self.owner.next(&self.account)? {
            solana_kit_client::subscriptions::Drain::Closed => Ok(()),
            _ => Err(Error::Subscription("Account unsubscribe did not settle").into()),
        }
    }

    async fn unsubscribe_all(&mut self, routing: &mut Routing<'_>) -> Result<(), AppError> {
        while self.socket.state() == SocketState::Connecting {
            self.receive(routing).await?;
        }

        loop {
            let account = self.owner.unsubscribe(&self.account, ACK_DEADLINE);
            let root = self.owner.unsubscribe(&self.root, ACK_DEADLINE);
            finish(account.map_err(Into::into), root.map_err(Into::into))?;
            self.flush().await?;

            if self.owner.settled(&self.account)? && self.owner.settled(&self.root)? {
                let account = self.owner.next(&self.account);
                let root = self.owner.next(&self.root);
                finish(
                    account.map_err(Into::into),
                    root.map(|_| ()).map_err(Into::into),
                )?;

                return Ok(());
            }

            self.receive(routing).await?;
        }
    }

    /// Both streams receive cancellation even when one fails, and socket close
    /// is attempted independently before the owner drops its cancellable lease.
    pub async fn close(&mut self, routing: &mut Routing<'_>) -> Result<(), AppError> {
        let subscriptions = timed(self.unsubscribe_all(routing)).await;
        let socket = if self.socket.state() == SocketState::Closed {
            Ok(())
        } else {
            timed(async { Ok(self.socket.close()?.await?) }).await
        };

        self.owner.terminate();
        finish(subscriptions, socket)
    }
}

fn active<R: SubscriptionMethod>(
    owner: &mut Connection,
    handle: &Subscription<R>,
) -> Result<bool, AppError> {
    match owner.status(handle)? {
        Status::Active => Ok(true),
        Status::Closed => {
            owner.next(handle)?;
            Err(Error::Subscription("Subscription ended before its acknowledgement").into())
        }
        _ => Ok(false),
    }
}
