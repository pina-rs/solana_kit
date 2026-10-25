//! Physical wire registrations outlive individual local consumers.

use super::{Connection, Entry, Failure, NotificationKind, Phase, SubscriptionMethod};
use crate::{Error, RemoteError};
use core::any::TypeId;
use std::sync::Arc;

pub(super) enum PendingFailure {
    Local(Failure),
    Remote(Arc<RemoteError>),
}

impl PendingFailure {
    pub(super) fn into_failure(self) -> Failure {
        match self {
            Self::Local(error) => error,
            Self::Remote(error) => {
                let error =
                    Arc::try_unwrap(error).unwrap_or_else(|error| clone_remote_error(&error));

                Failure::Protocol(Error::Remote(error))
            }
        }
    }
}

fn clone_remote_error(error: &RemoteError) -> RemoteError {
    let data = match &error.data {
        crate::Presence::Missing => crate::Presence::Missing,
        crate::Presence::Present(value) => crate::Presence::Present(value.clone()),
    };

    RemoteError {
        code: error.code,
        message: error.message.clone(),
        data,
    }
}

#[derive(PartialEq, Eq)]
pub(super) struct SharingKey {
    schema: TypeId,
    params: Vec<u8>,
    subscribe: &'static str,
    unsubscribe: &'static str,
    notification: &'static str,
    kind: NotificationKind,
}

impl SharingKey {
    pub(super) fn new<R: SubscriptionMethod>(body: &[u8]) -> Result<Option<Self>, Failure> {
        let Some(schema) = R::sharing_identity() else {
            return Ok(None);
        };
        crate::validate::unique(body).map_err(Error::Json)?;
        let mut value: serde_json::Value = serde_json::from_slice(body).map_err(Error::Json)?;
        value.sort_all_objects();
        let params = value.get("params").ok_or(Failure::Method)?;

        Ok(Some(Self {
            schema,
            params: serde_json::to_vec(params).map_err(Error::Json)?,
            subscribe: R::NAME,
            unsubscribe: R::UNSUBSCRIBE,
            notification: R::NOTIFICATION,
            kind: R::NOTIFICATION_KIND,
        }))
    }
}

pub(super) struct Physical {
    pub(super) phase: Phase,
    pub(super) sharing: Option<SharingKey>,
    pub(super) schema: Option<TypeId>,
    pub(super) subscribe: &'static str,
    pub(super) unsubscribe: &'static str,
    pub(super) notification: &'static str,
    pub(super) kind: NotificationKind,
    pub(super) quarantined: bool,
    pub(super) reserved_server: Option<u64>,
}

impl Physical {
    pub(super) fn server(&self) -> Option<u64> {
        self.phase.server().or(self.reserved_server)
    }

    fn share(&mut self, deadline: u64) -> Result<bool, Failure> {
        if self.quarantined {
            return Err(Failure::Closing);
        }

        match &mut self.phase {
            Phase::Unsubscribing { .. } => Err(Failure::Closing),
            Phase::Subscribing {
                deadline: existing, ..
            } => {
                *existing = (*existing).max(deadline);
                Ok(true)
            }
            Phase::Active(_) => Ok(true),
            Phase::Ended => Ok(false),
        }
    }

    fn expire(&mut self, now: u64) -> bool {
        if !self.phase.expired(now) {
            return false;
        }

        if matches!(self.phase, Phase::Subscribing { .. }) && self.sharing.is_some() {
            self.quarantined = true;
        }

        self.reserved_server = if self.quarantined {
            self.phase.server()
        } else {
            None
        };
        self.phase = Phase::Ended;
        true
    }
}

impl Connection {
    pub(super) fn registration<R: SubscriptionMethod>(
        &mut self,
        key: u32,
        deadline: u64,
        sharing: Option<SharingKey>,
        body: Vec<u8>,
    ) -> Result<u32, Failure> {
        if let Some(existing) = self.shared_registration(sharing.as_ref(), deadline)? {
            return Ok(existing);
        }

        if self.physical.len() >= self.budget.subscriptions {
            return Err(Failure::Capacity);
        }

        self.enqueue(body)?;
        self.physical.insert(
            key,
            Physical {
                phase: Phase::Subscribing {
                    request: key,
                    deadline,
                },
                sharing,
                schema: R::schema_identity(),
                subscribe: R::NAME,
                unsubscribe: R::UNSUBSCRIBE,
                notification: R::NOTIFICATION,
                kind: R::NOTIFICATION_KIND,
                quarantined: false,
                reserved_server: None,
            },
        );

        Ok(key)
    }

    fn shared_registration(
        &mut self,
        sharing: Option<&SharingKey>,
        deadline: u64,
    ) -> Result<Option<u32>, Failure> {
        let Some(sharing) = sharing else {
            return Ok(None);
        };

        for (key, registration) in &mut self.physical {
            if registration.sharing.as_ref() == Some(sharing) && registration.share(deadline)? {
                return Ok(Some(*key));
            }
        }

        Ok(None)
    }

    pub(super) fn has_receivers(&self, physical: u32) -> bool {
        self.entries
            .values()
            .any(|entry| entry.physical == physical && receives(entry))
    }

    pub(super) fn receivers(&self, physical: u32) -> Vec<u32> {
        self.entries
            .iter()
            .filter_map(|(key, entry)| {
                (entry.physical == physical && receives(entry)).then_some(*key)
            })
            .collect()
    }

    /// All local queues must fit before counters change, so fanout cannot be partial.
    pub(super) fn preflight(&mut self, keys: &[u32], bytes: usize) -> Result<(), Failure> {
        let total = bytes.checked_mul(keys.len()).ok_or(Failure::Overflow)?;

        if keys.len() > self.budget.messages.saturating_sub(self.messages)
            || total > self.budget.queued_bytes.saturating_sub(self.bytes)
        {
            return Err(Failure::Overflow);
        }

        for key in keys {
            let entry = self.entries.get(key).ok_or(Failure::Closed)?;

            if entry.queue.len() >= entry.budget.messages
                || bytes > entry.budget.bytes.saturating_sub(entry.bytes)
            {
                return Err(Failure::Overflow);
            }
        }

        self.messages += keys.len();
        self.bytes += total;

        Ok(())
    }

    pub(super) fn detach_if_shared(&mut self, key: u32) -> Result<(), Failure> {
        let entry = self.entries.get(&key).ok_or(Failure::Closed)?;
        let physical = entry.physical;

        if entry.cancelled && self.has_receivers(physical) {
            self.entries.get_mut(&key).ok_or(Failure::Closed)?.ended = true;
        }

        Ok(())
    }

    pub(super) fn tick_physical(&mut self, now: u64, deadline: u64) -> Result<(), Failure> {
        let keys: Vec<_> = self.physical.keys().copied().collect();

        for key in keys {
            let expired = self
                .physical
                .get_mut(&key)
                .ok_or(Failure::Closed)?
                .expire(now);

            if expired {
                self.expire_receivers(key);
            }

            self.start_unsubscribe(key, deadline)?;
            self.remove_physical_if_unused(key);
        }

        Ok(())
    }

    fn expire_receivers(&mut self, key: u32) {
        for entry in self
            .entries
            .values_mut()
            .filter(|entry| entry.physical == key && !entry.ended)
        {
            entry.ended = true;
            entry.failure = Some(PendingFailure::Local(Failure::Deadline));
        }
    }

    pub(super) fn remove_physical_if_unused(&mut self, key: u32) {
        let abandoned: Vec<_> = self
            .entries
            .iter()
            .filter_map(|(key, entry)| (entry.ended && entry.abandoned()).then_some(*key))
            .collect();

        for abandoned in abandoned {
            self.remove_abandoned(abandoned);
        }

        let terminal = self
            .physical
            .get(&key)
            .is_some_and(|entry| matches!(entry.phase, Phase::Ended));

        let certain = self
            .physical
            .get(&key)
            .is_some_and(|entry| !entry.quarantined);

        if terminal && certain && !self.entries.values().any(|entry| entry.physical == key) {
            self.physical.remove(&key);
        }
    }

    fn remove_abandoned(&mut self, key: u32) {
        let Some(entry) = self.entries.remove(&key) else {
            return;
        };

        for payload in entry.queue {
            self.release(payload.len());
        }
    }

    pub(super) fn remote_error(&mut self, key: u32, error: RemoteError) -> Result<(), Failure> {
        let registration = self.physical.get_mut(&key).ok_or(Failure::Closed)?;
        registration.reserved_server = if registration.quarantined {
            registration.phase.server()
        } else {
            None
        };

        registration.phase = Phase::Ended;
        let error = Arc::new(error);

        for entry in self
            .entries
            .values_mut()
            .filter(|entry| entry.physical == key && !entry.ended)
        {
            entry.ended = true;
            entry.failure = Some(PendingFailure::Remote(Arc::clone(&error)));
        }

        Ok(())
    }
}

pub(super) fn receives(entry: &Entry) -> bool {
    !entry.cancelled && !entry.ended
}
