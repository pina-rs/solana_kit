//! Session correlation and media hydration do not imply transaction or publication approval.

use super::{
    Effect, Error, Snapshot, Stage, Storage, Workflow,
    engine::bind,
    media::{self, Document, MediaInput, ResolvedMedia, Spec},
    snapshot::{UploadIntent, UploadReceipt},
    source::{request_upload_target, sha256},
    text, value,
};
use crate::{
    deadline,
    portal::{
        self, BackendPublicationBundle, BackendPublicationSession, CreateUploadTarget,
        FetchRemoteFile, GetPublicationBundle, GetPublicationSession, PortalService, UploadRequest,
        Uploader,
    },
};
use solana_kit_client::{rest::Transport, subscriptions::Clock};
use solana_kit_core::codecs::{Base64, Encoder};

/// Hydrated identifiers and metadata URI alongside the original typed backend proposals.
///
/// This is an ingestion/hydration result, not a transaction-policy seal or store receipt.
pub struct HydratedPublication {
    release_id: String,
    ingestion_id: String,
    publication_id: String,
    metadata_uri: String,
    stage: Stage,
    bundle: BackendPublicationBundle,
    session: BackendPublicationSession,
}

impl HydratedPublication {
    /// Correlated release identity.
    pub fn release_id(&self) -> &str {
        &self.release_id
    }
    /// Correlated ingestion identity.
    pub fn ingestion_id(&self) -> &str {
        &self.ingestion_id
    }
    /// Correlated publication-session identity.
    pub fn publication_session_id(&self) -> &str {
        &self.publication_id
    }
    /// Existing or durably acknowledged uploaded metadata location.
    pub fn metadata_uri(&self) -> &str {
        &self.metadata_uri
    }
    /// Backend stage resolved with the pinned receipt fallbacks.
    pub const fn stage(&self) -> Stage {
        self.stage
    }
    /// Unmodified backend bundle, without silently replacing its missing fields.
    pub const fn backend_bundle(&self) -> &BackendPublicationBundle {
        &self.bundle
    }
    /// Unmodified backend session, including raw timestamps and optional receipts.
    pub const fn backend_session(&self) -> &BackendPublicationSession {
        &self.session
    }
}

impl<T, U, S, C> Workflow<'_, T, U, S, C>
where
    T: Transport<PortalService>,
    U: Uploader,
    S: Storage,
    C: Clock,
{
    /// Session and bundle admission are separate so untrusted metadata cannot enter the cache.
    pub(super) async fn hydrate(
        &mut self,
        snapshot: &mut Snapshot,
        attached_bundle: Option<BackendPublicationBundle>,
        attached_session: Option<BackendPublicationSession>,
    ) -> Result<HydratedPublication, Error> {
        let session = match attached_session {
            Some(session) => session,
            None => {
                self.call(&GetPublicationSession {
                    publication_session_id: snapshot.publication_id.clone(),
                    release_id: snapshot
                        .release_id
                        .clone()
                        .filter(|_| snapshot.publication_id.is_none()),
                })
                .await?
            }
        };

        let stage = correlate_session(snapshot, &session)?;

        if let Some(uri) = text(&session.metadata_uri) {
            media::urls::public(uri)?;
        }

        snapshot
            .updated_on
            .get_or_insert_with(|| self.options.updated_on.clone());
        self.save(snapshot).await?;

        if stage == Stage::Failed {
            return Err(Error::PublicationFailed(Box::new(session)));
        }

        if snapshot.preserved_submitted && stage != Stage::Submitted {
            return Err(Error::Invalid("preserved release is not submitted"));
        }

        let bundle = self
            .load_bundle(snapshot, attached_bundle, text(&session.metadata_uri))
            .await?;
        correlate(snapshot, &bundle)?;
        let metadata_uri = self
            .metadata_uri(snapshot, &session, &bundle, stage)
            .await?;
        snapshot.metadata_uri = Some(metadata_uri.clone());
        snapshot.pending = None;
        self.save(snapshot).await?;

        HydratedPublication::new(snapshot, metadata_uri, stage, bundle, session)
    }

    async fn load_bundle(
        &self,
        snapshot: &Snapshot,
        attached: Option<BackendPublicationBundle>,
        session_uri: Option<&str>,
    ) -> Result<BackendPublicationBundle, Error> {
        let existing_uri = snapshot.metadata_uri.as_deref().or(session_uri);
        let attached =
            attached.filter(|bundle| existing_uri.is_some() || backend_uri(bundle).is_some());

        let bundle = match attached {
            Some(bundle) => bundle,
            None => {
                self.call(&GetPublicationBundle {
                    release_id: snapshot
                        .release_id
                        .clone()
                        .ok_or(Error::Invalid("missing release identity"))?,
                })
                .await?
            }
        };

        if let Some(uri) = backend_uri(&bundle) {
            media::urls::public(uri)?;
        }

        Ok(bundle)
    }

    async fn metadata_uri(
        &mut self,
        snapshot: &mut Snapshot,
        session: &BackendPublicationSession,
        bundle: &BackendPublicationBundle,
        stage: Stage,
    ) -> Result<String, Error> {
        let uri = snapshot
            .metadata_uri
            .as_deref()
            .or_else(|| text(&session.metadata_uri))
            .or_else(|| backend_uri(&bundle));

        if let Some(uri) = uri {
            return media::urls::public(uri);
        }

        if stage == Stage::Submitted {
            return Err(Error::Invalid("submitted release lacks metadata URI"));
        }

        self.upload_metadata(snapshot, bundle).await
    }

    async fn upload_metadata(
        &mut self,
        snapshot: &mut Snapshot,
        bundle: &BackendPublicationBundle,
    ) -> Result<String, Error> {
        let spec = Spec::new(
            bundle,
            snapshot
                .updated_on
                .as_deref()
                .ok_or(Error::Invalid("missing metadata timestamp"))?,
            self.options.limits.media_items,
        )?;
        let mut media = Vec::with_capacity(spec.media.len());

        for input in &spec.media {
            media.push(self.resolve_media(snapshot, input).await?);
        }

        let document = Document::new(&spec, &media)?;
        let bytes = self
            .options
            .limits
            .metadata
            .serialize(&document)
            .map_err(|error| Error::Portal(portal::Error::Json(error)))?;
        self.upload(
            snapshot,
            &bytes,
            "json",
            "application/json",
            self.options.limits.metadata.bytes,
        )
        .await
    }

    /// R2 reuse still requires identical byte/header checks; only the public location avoids PUT.
    async fn resolve_media(
        &mut self,
        snapshot: &mut Snapshot,
        input: &MediaInput,
    ) -> Result<ResolvedMedia, Error> {
        let request = FetchRemoteFile {
            url: input.uri.clone(),
            file_name: Some(input.fallback_name.clone()),
            expected_mime_type: None,
        };
        let payload = self.call(&request).await?;
        let bytes = Base64
            .encode(payload.data.as_str(), self.options.limits.media_bytes)
            .map_err(Error::Codec)?;

        if bytes.is_empty() {
            return Err(Error::Invalid("empty remote media"));
        }

        let file_name = if payload.file_name.is_empty() {
            &input.fallback_name
        } else {
            &payload.file_name
        };

        let mime = media::mime::normalized(&payload.mime_type)
            .or_else(|| input.default_mime.map(str::to_owned))
            .unwrap_or_else(|| media::mime::infer(file_name).into());
        media::mime::validate(&mime)?;
        let dimensions = media::read_image_dimensions(&bytes)?;
        let sha256 = sha256(&bytes)?;
        let uri = if media::urls::public_r2(&input.uri) {
            input.uri.clone()
        } else {
            let extension = media::mime::extension(file_name, &mime);
            self.upload(
                snapshot,
                &bytes,
                &extension,
                &mime,
                self.options.limits.media_bytes,
            )
            .await?
        };

        Ok(ResolvedMedia {
            mime,
            purpose: input.purpose,
            width: dimensions.width,
            height: dimensions.height,
            sha256,
            uri,
        })
    }

    async fn upload(
        &mut self,
        snapshot: &mut Snapshot,
        bytes: &[u8],
        extension: &str,
        mime: &str,
        maximum: usize,
    ) -> Result<String, Error> {
        let hash = sha256(bytes)?;
        let request = CreateUploadTarget {
            file_hash: hash.clone(),
            file_extension: extension.into(),
            content_type: mime.into(),
        };

        if let Some(receipt) = snapshot.uploads.iter().find(|receipt| {
            receipt.hash == request.file_hash
                && receipt.mime == mime
                && receipt.extension == extension
        }) {
            return media::urls::public(&receipt.public_url);
        }

        snapshot.pending_upload = Some(UploadIntent {
            hash: request.file_hash.clone(),
            mime: mime.into(),
            extension: extension.into(),
            public_url: None,
        });
        self.intent(snapshot, Effect::CreateUploadTarget).await?;
        let target = request_upload_target(
            self.client,
            self.clock,
            &request,
            self.options.limits.deadline,
        )
        .await?;
        let public_url = target.public_url.clone();
        snapshot
            .pending_upload
            .as_mut()
            .ok_or(Error::Invalid("missing upload intent"))?
            .public_url = Some(public_url.clone());
        self.intent(snapshot, Effect::Upload).await?;
        let request = UploadRequest {
            target: &target,
            body: bytes,
            content_type: mime,
            max_bytes: maximum,
        };
        deadline::within(
            portal::upload_bytes(self.uploader, request),
            self.clock,
            self.options.limits.deadline,
        )
        .await
        .map_err(Error::from)?
        .map_err(Error::Portal)?;
        snapshot.pending = None;
        snapshot.pending_upload = None;
        snapshot.uploads.push(UploadReceipt {
            hash,
            mime: mime.into(),
            extension: extension.into(),
            public_url: public_url.clone(),
        });
        self.save(snapshot).await?;
        Ok(public_url)
    }
}

impl HydratedPublication {
    fn new(
        snapshot: &Snapshot,
        metadata_uri: String,
        stage: Stage,
        bundle: BackendPublicationBundle,
        session: BackendPublicationSession,
    ) -> Result<Self, Error> {
        Ok(Self {
            release_id: snapshot
                .release_id
                .clone()
                .ok_or(Error::Invalid("missing release identity"))?,
            ingestion_id: snapshot
                .ingestion_id
                .clone()
                .ok_or(Error::Invalid("missing ingestion identity"))?,
            publication_id: snapshot
                .publication_id
                .clone()
                .ok_or(Error::Invalid("missing publication identity"))?,
            metadata_uri,
            stage,
            bundle,
            session,
        })
    }
}

fn correlate_session(
    snapshot: &mut Snapshot,
    session: &BackendPublicationSession,
) -> Result<Stage, Error> {
    bind(
        &mut snapshot.publication_id,
        text(&session.id).ok_or(Error::Invalid("missing publication identity"))?,
    )?;
    bind(
        &mut snapshot.release_id,
        text(&session.release_id).ok_or(Error::Invalid("missing session release identity"))?,
    )?;
    bind(
        &mut snapshot.ingestion_id,
        text(&session.ingestion_session_id)
            .ok_or(Error::Invalid("missing session ingestion identity"))?,
    )?;
    let stage = Stage::resolve(session)?;

    Ok(stage)
}

fn backend_uri(bundle: &BackendPublicationBundle) -> Option<&str> {
    value(&bundle.release).and_then(|release| text(&release.nft_metadata_uri))
}

fn correlate(snapshot: &mut Snapshot, bundle: &BackendPublicationBundle) -> Result<(), Error> {
    if let Some(id) = text(&bundle.release_id) {
        bind(&mut snapshot.release_id, id)?;
    }

    if let Some(id) = value(&bundle.release).and_then(|release| text(&release.id)) {
        bind(&mut snapshot.release_id, id)?;
    }

    if let Some(id) = text(&bundle.ingestion_session_id) {
        bind(&mut snapshot.ingestion_id, id)?;
    }

    if let Some(id) = text(&bundle.publication_session_id) {
        bind(&mut snapshot.publication_id, id)?;
    }

    Ok(())
}
