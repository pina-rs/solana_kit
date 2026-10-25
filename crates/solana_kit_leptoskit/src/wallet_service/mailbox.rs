//! Retain one current operation while a cancelled generation leaves the worker queue.
use super::worker::Command;
use leptoskit_bridge::Error;
use std::{future::Future, pin::Pin, task::Context};
use tokio::sync::mpsc::{OwnedPermit, Sender, error::SendError, error::TrySendError};

type Reservation =
    Pin<Box<dyn Future<Output = Result<OwnedPermit<Command>, SendError<()>>> + Send>>;

struct Queued {
    command: Command,
    capacity: Reservation,
}

pub(super) struct Mailbox {
    sender: Sender<Command>,
    queued: Option<Queued>,
}

impl Mailbox {
    pub(super) fn new(sender: Sender<Command>) -> Self {
        Self {
            sender,
            queued: None,
        }
    }

    pub(super) fn send(&mut self, command: Command) -> Result<(), Error> {
        if self.queued.is_some() {
            return Err(Error::Busy);
        }

        match self.sender.try_send(command) {
            Ok(()) => Ok(()),
            Err(TrySendError::Closed(_)) => Err(Error::Native),
            Err(TrySendError::Full(command)) => {
                self.queued = Some(Queued {
                    command,
                    capacity: Box::pin(self.sender.clone().reserve_owned()),
                });
                Ok(())
            }
        }
    }

    pub(super) fn flush(&mut self, context: &mut Context<'_>) -> Result<(), Error> {
        let Some(queued) = self.queued.as_mut() else {
            return Ok(());
        };

        let permit = match queued.capacity.as_mut().poll(context) {
            std::task::Poll::Pending => return Ok(()),
            std::task::Poll::Ready(permit) => permit,
        };
        let queued = self.queued.take().ok_or(Error::Native)?;
        drop(permit.map_err(|_| Error::Native)?.send(queued.command));
        Ok(())
    }

    pub(super) fn clear(&mut self) {
        self.queued = None;
    }
}

#[cfg(test)]
mod tests;
