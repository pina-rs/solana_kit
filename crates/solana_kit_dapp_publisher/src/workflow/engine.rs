//! Polling and recovery never replay a persisted uncertain mutation.

use super::{
    Effect, Error, HydratedPublication, Options, ResumeTarget, Snapshot, Storage, source::sha256,
    text,
};
use crate::{
    deadline,
    portal::{
        self, CleanupRelease, CleanupReleaseResult, Client, CreateIngestionSession,
        GetIngestionSession, Operation, PortalService, Uploader,
    },
};
use solana_kit_client::{rest::Transport, subscriptions::Clock};

/// One exclusive caller-driven ingestion/hydration coordinator; it owns no runtime or signer.
pub struct Workflow<'a, T, U, S, C> {
    pub(super) client: &'a Client<T>,
    pub(super) uploader: &'a U,
    pub(super) storage: &'a mut S,
    pub(super) clock: &'a C,
    pub(super) options: Options,
}

impl<'a, T, U, S, C> Workflow<'a, T, U, S, C>
where
    T: Transport<PortalService>,
    U: Uploader,
    S: Storage,
    C: Clock,
{
    /// Validates local policy without reading storage, files, credentials or network.
    pub fn new(
        client: &'a Client<T>,
        uploader: &'a U,
        storage: &'a mut S,
        clock: &'a C,
        options: Options,
    ) -> Result<Self, Error> {
        options.limits.validate()?;

        if options.snapshot_key.trim().is_empty() || !super::timestamp::valid(&options.updated_on) {
            return Err(Error::Invalid(
                "invalid checkpoint key or UTC metadata timestamp",
            ));
        }

        Ok(Self {
            client,
            uploader,
            storage,
            clock,
            options,
        })
    }

    /// Creates ingestion once, polls within bounds, then hydrates metadata for caller review.
    ///
    /// The caller supplies a fresh idempotency key and already remote source.
    /// Existing checkpoints require explicit resume; even a lost create response
    /// never causes an automatic replay here. No transaction is signed or submitted.
    pub async fn start(
        &mut self,
        input: &CreateIngestionSession,
    ) -> Result<HydratedPublication, Error> {
        let clock = self.clock;
        let deadline = self.options.limits.deadline;
        deadline::within(self.start_inner(input), clock, deadline)
            .await
            .map_err(Error::from)?
    }

    async fn start_inner(
        &mut self,
        input: &CreateIngestionSession,
    ) -> Result<HydratedPublication, Error> {
        if input.idempotency_key.trim().is_empty() {
            return Err(Error::Invalid("missing idempotency key"));
        }

        if self.load().await?.is_some() {
            return Err(Error::Invalid("existing checkpoint requires resume"));
        }

        let mut snapshot = Snapshot::new();
        let hash = async {
            let bytes = self
                .options
                .limits
                .snapshot
                .serialize(input)
                .map_err(|error| Error::Portal(portal::Error::Json(error)))?;
            sha256(&bytes)
        };
        snapshot.input_hash = Some(
            deadline::within(hash, self.clock, self.options.limits.deadline)
                .await
                .map_err(Error::from)??,
        );
        snapshot.idempotency_key = Some(input.idempotency_key.clone());
        snapshot.updated_on = Some(self.options.updated_on.clone());
        self.intent(&mut snapshot, Effect::CreateIngestion).await?;
        let session = self.call(input).await?;
        snapshot.ingestion_id = Some(
            text(&session.id)
                .ok_or(Error::Invalid("missing ingestion identity"))?
                .into(),
        );
        snapshot.release_id = text(&session.release_id).map(str::to_owned);
        snapshot.publication_id = text(&session.publication_session_id).map(str::to_owned);
        snapshot.pending = None;
        self.save(&snapshot).await?;
        self.poll(&mut snapshot).await
    }

    /// Reconciles acknowledged state by snapshot, release or publication-session identity.
    ///
    /// A pending checkpoint returns Uncertain before further network work. The
    /// caller must reconcile that effect with its provider/storage authority;
    /// changing a selector cannot silently bypass a pending effect.
    pub async fn resume(&mut self, target: ResumeTarget) -> Result<HydratedPublication, Error> {
        let clock = self.clock;
        let deadline = self.options.limits.deadline;
        deadline::within(self.resume_inner(target), clock, deadline)
            .await
            .map_err(Error::from)?
    }

    async fn resume_inner(&mut self, target: ResumeTarget) -> Result<HydratedPublication, Error> {
        let mut snapshot = self.load().await?.unwrap_or_else(Snapshot::new);
        snapshot.admit()?;

        match target {
            ResumeTarget::Snapshot => {}
            ResumeTarget::Session(id) => bind(&mut snapshot.publication_id, &id)?,
            ResumeTarget::Release(id) => bind(&mut snapshot.release_id, &id)?,
        }

        if snapshot.release_id.is_none() && snapshot.publication_id.is_none() {
            if snapshot.ingestion_id.is_some() {
                return self.poll(&mut snapshot).await;
            }

            return Err(Error::Invalid("checkpoint has no recovery identity"));
        }

        self.hydrate(&mut snapshot, None, None).await
    }

    /// Explicitly abandons a failed release with one cleanup call; never runs from Drop.
    ///
    /// A prior uncertain cleanup cannot be replayed. PreservedSubmitted records
    /// a requirement to reconcile a submitted session; it never resubmits to store.
    pub async fn cleanup_failed(&mut self) -> Result<CleanupReleaseResult, Error> {
        let clock = self.clock;
        let deadline = self.options.limits.deadline;
        deadline::within(self.cleanup_inner(), clock, deadline)
            .await
            .map_err(Error::from)?
    }

    async fn cleanup_inner(&mut self) -> Result<CleanupReleaseResult, Error> {
        let mut snapshot = self
            .load()
            .await?
            .ok_or(Error::Invalid("missing checkpoint"))?;

        if snapshot.pending == Some(Effect::Cleanup) {
            return Err(Error::Uncertain(Effect::Cleanup));
        }

        if snapshot.deleted || snapshot.preserved_submitted {
            return Err(Error::Invalid("cleanup already acknowledged"));
        }

        let release_id = snapshot
            .release_id
            .clone()
            .ok_or(Error::Invalid("cleanup requires known release"))?;
        snapshot.pending_upload = None;
        self.intent(&mut snapshot, Effect::Cleanup).await?;
        let result = self.call(&CleanupRelease { release_id }).await?;

        match result.action.as_str() {
            "deleted" => snapshot.deleted = true,
            "preservedSubmitted" => snapshot.preserved_submitted = true,
            _ => return Err(Error::Invalid("unknown cleanup outcome")),
        }

        snapshot.pending = None;
        snapshot.pending_upload = None;
        self.save(&snapshot).await?;
        Ok(result)
    }

    async fn poll(&mut self, snapshot: &mut Snapshot) -> Result<HydratedPublication, Error> {
        let session = self.poll_ready(snapshot).await?;

        if snapshot.release_id.is_none() {
            return Err(Error::Invalid("ready ingestion lacks release identity"));
        }

        self.hydrate(
            snapshot,
            super::take(session.bundle),
            super::take(session.publication_session),
        )
        .await
    }

    /// Readiness is bounded independently of retries; failed sessions retain their cleanup identity.
    async fn poll_ready(
        &mut self,
        snapshot: &mut Snapshot,
    ) -> Result<portal::BackendIngestionSession, Error> {
        let session_id = snapshot
            .ingestion_id
            .clone()
            .ok_or(Error::Invalid("missing ingestion identity"))?;

        for attempt in 0..self.options.limits.max_polls {
            let session = self
                .call(&GetIngestionSession {
                    session_id: session_id.clone(),
                })
                .await?;
            record_ingestion(snapshot, &session, &session_id)?;
            self.save(snapshot).await?;

            match text(&session.status) {
                Some("Failed" | "failed") => return Err(Error::IngestionFailed(Box::new(session))),
                Some("Ready" | "ready") => return Ok(session),
                _ => {}
            }

            if attempt + 1 == self.options.limits.max_polls {
                return Err(Error::PollLimit);
            }

            self.wait_before_poll().await?;
        }

        Err(Error::PollLimit)
    }

    async fn wait_before_poll(&self) -> Result<(), Error> {
        let wake = self
            .clock
            .now()
            .checked_add(self.options.limits.poll_interval_ms)
            .ok_or(Error::Invalid("poll interval overflow"))?;

        if wake >= self.options.limits.deadline {
            return Err(Error::Deadline);
        }

        deadline::within(
            self.clock.sleep_until(wake),
            self.clock,
            self.options.limits.deadline,
        )
        .await
        .map_err(Error::from)?
        .map_err(Error::Clock)
    }

    pub(super) async fn call<O: Operation>(&self, request: &O) -> Result<O::Response, Error> {
        deadline::within(
            self.client.call(request),
            self.clock,
            self.options.limits.deadline,
        )
        .await
        .map_err(Error::from)?
        .map_err(Error::Portal)
    }

    pub(super) async fn load(&self) -> Result<Option<Snapshot>, Error> {
        let load = async {
            let bytes = self
                .storage
                .load(
                    &self.options.snapshot_key,
                    self.options.limits.snapshot.bytes,
                )
                .await
                .map_err(Error::Storage)?;
            bytes
                .map(|bytes| Snapshot::decode(&bytes, self.options.limits.snapshot))
                .transpose()
        };
        deadline::within(load, self.clock, self.options.limits.deadline)
            .await
            .map_err(Error::from)?
    }

    pub(super) async fn save(&mut self, snapshot: &Snapshot) -> Result<(), Error> {
        let clock = self.clock;
        let deadline = self.options.limits.deadline;
        let budgets = self.options.limits.snapshot;
        let key = &self.options.snapshot_key;
        let storage = &mut *self.storage;
        let save = async {
            let bytes = budgets
                .serialize(snapshot)
                .map_err(|error| Error::Portal(portal::Error::Json(error)))?;
            storage
                .save(key, &bytes, budgets.bytes)
                .await
                .map_err(Error::Storage)
        };
        deadline::within(save, clock, deadline)
            .await
            .map_err(Error::from)?
    }

    pub(super) async fn intent(
        &mut self,
        snapshot: &mut Snapshot,
        effect: Effect,
    ) -> Result<(), Error> {
        snapshot.pending = Some(effect);
        self.save(snapshot).await
    }
}

fn record_ingestion(
    snapshot: &mut Snapshot,
    session: &portal::BackendIngestionSession,
    expected_id: &str,
) -> Result<(), Error> {
    if text(&session.id) != Some(expected_id) {
        return Err(Error::Invalid("ingestion identity mismatch"));
    }

    if snapshot
        .idempotency_key
        .as_deref()
        .zip(text(&session.idempotency_key))
        .is_some_and(|(expected, actual)| expected != actual)
    {
        return Err(Error::Invalid("ingestion key mismatch"));
    }

    if let Some(id) = text(&session.release_id) {
        bind(&mut snapshot.release_id, id)?;
    }

    if let Some(id) = text(&session.publication_session_id) {
        bind(&mut snapshot.publication_id, id)?;
    }

    Ok(())
}

pub(super) fn bind(destination: &mut Option<String>, value: &str) -> Result<(), Error> {
    if value.trim().is_empty()
        || destination
            .as_deref()
            .is_some_and(|expected| expected != value)
    {
        return Err(Error::Invalid("publication identity mismatch"));
    }

    *destination = Some(value.into());
    Ok(())
}
