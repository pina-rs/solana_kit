//! Bounded application timers share the existing Bridge routing owner.
use crate::{Host, native};
use leptoskit_bridge::{Bytes, CallId, Error, MethodId, PluginId};
use solana_kit_leptoskit::Routed;
use std::{
    collections::BTreeMap,
    task::{Context, Poll},
    time::Duration,
};
use tokio::task::{AbortHandle, Id, JoinSet};

const SLEEP: MethodId = MethodId(1);
const MAX_TIMERS: usize = 8;
const MAX_MILLIS: u64 = 60_000;

pub(super) struct Timers {
    plugin: PluginId,
    jobs: BTreeMap<Id, (CallId, AbortHandle)>,
    tasks: JoinSet<CallId>,
}

impl Timers {
    pub(super) fn new(host: &mut Host) -> Result<Self, Error> {
        let plugin = host.plugins().register("journey.timer", [SLEEP])?;
        host.plugins().grant(plugin, SLEEP, true)?;

        Ok(Self {
            plugin,
            jobs: BTreeMap::new(),
            tasks: JoinSet::new(),
        })
    }

    /// Cancel first because a full admission budget must still permit teardown.
    pub(super) fn route(&mut self, host: &mut Host, mut routed: Routed) -> Result<Routed, Error> {
        routed.cancelled.retain(|call| {
            let job = self.jobs.values().find(|(owned, _)| owned == call);

            if let Some((_, abort)) = job {
                abort.abort();
            }

            job.is_none()
        });
        let mut unrelated = Vec::new();

        for request in routed.requests {
            if request.plugin != self.plugin {
                unrelated.push(request);
                continue;
            }

            let value = request
                .payload
                .as_ref()
                .try_into()
                .ok()
                .map(u64::from_le_bytes);
            let Some(millis) = value.filter(|value| *value <= MAX_MILLIS) else {
                host.plugins()
                    .complete(request.id, Err(Error::InvalidArgument))?;
                routed.guest_woken = true;
                continue;
            };

            if self.jobs.len() >= MAX_TIMERS {
                host.plugins().complete(request.id, Err(Error::Busy))?;
                routed.guest_woken = true;
                continue;
            }

            let abort = self.tasks.spawn(async move {
                tokio::time::sleep(Duration::from_millis(millis)).await;
                request.id
            });
            self.jobs.insert(abort.id(), (request.id, abort));
        }

        routed.requests = unrelated;

        if routed.guest_woken {
            host.wake_plugins().map_err(native)?;
        }

        Ok(routed)
    }

    pub(super) fn poll(
        &mut self,
        host: &mut Host,
        context: &mut Context<'_>,
    ) -> Result<bool, Error> {
        let Poll::Ready(Some(result)) = self.tasks.poll_join_next_with_id(context) else {
            return Ok(false);
        };
        let id = result
            .as_ref()
            .map(|(id, _)| *id)
            .unwrap_or_else(|error| error.id());
        self.jobs.remove(&id).ok_or(Error::Native)?;
        check_join(result.as_ref().err())?;

        if let Ok((_, call)) = result {
            match host.plugins().complete(call, Ok(Bytes::new())) {
                Ok(()) => host.wake_plugins().map_err(native)?,
                Err(Error::InvalidCall) => {}
                Err(error) => return Err(error),
            }
        }

        Ok(true)
    }

    pub(super) async fn close(&mut self) -> Result<(), Error> {
        self.tasks.abort_all();
        let mut panicked = false;

        while let Some(result) = self.tasks.join_next_with_id().await {
            let id = result
                .as_ref()
                .map(|(id, _)| *id)
                .unwrap_or_else(|error| error.id());
            self.jobs.remove(&id).ok_or(Error::Native)?;
            panicked |= check_join(result.as_ref().err()).is_err();
        }

        if panicked || !self.jobs.is_empty() {
            return Err(Error::Native);
        }

        Ok(())
    }
}

fn check_join(error: Option<&tokio::task::JoinError>) -> Result<(), Error> {
    if error.is_some_and(|error| !error.is_cancelled()) {
        return Err(Error::Native);
    }

    Ok(())
}
