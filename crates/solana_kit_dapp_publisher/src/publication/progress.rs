//! Caller-facing progress and status projection from the pinned logging flow.
//!
//! Provider detail, stage and error strings are deliberately absent from these
//! messages. They remain explicit data on the normalized session so ordinary
//! console output cannot echo arbitrary portal text or application metadata.

use super::PublicationIngestionSession;

/// One stable publisher progress step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressStep {
    /// Preparing the portal-visible source.
    SourcePrepare,
    /// Creating the ingestion session.
    IngestionCreate,
    /// Waiting for portal ingestion.
    IngestionWait,
    /// Loading the publication session.
    SessionLoad,
    /// Loading the publication bundle.
    BundleLoad,
    /// Recovering an already-submitted publication.
    CleanupRecover,
    /// Rolling back a failed release.
    CleanupRelease,
    /// Preparing the release mint transaction.
    MintPrepare,
    /// Reporting a submitted release mint transaction.
    MintSubmit,
    /// Saving release NFT data.
    MintSave,
    /// Preparing collection verification.
    VerifyPrepare,
    /// Submitting and acknowledging collection verification.
    VerifySubmit,
    /// Creating the attestation payload.
    AttestationCreate,
    /// Submitting the release to store review.
    SubmitStore,
}

impl ProgressStep {
    /// Wire step name used by the pinned source logger.
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::SourcePrepare => "source.prepare",
            Self::IngestionCreate => "ingestion.create",
            Self::IngestionWait => "ingestion.wait",
            Self::SessionLoad => "session.load",
            Self::BundleLoad => "bundle.load",
            Self::CleanupRecover => "cleanup.recover",
            Self::CleanupRelease => "cleanup.release",
            Self::MintPrepare => "mint.prepare",
            Self::MintSubmit => "mint.submit",
            Self::MintSave => "mint.save",
            Self::VerifyPrepare => "verify.prepare",
            Self::VerifySubmit => "verify.submit",
            Self::AttestationCreate => "attestation.create",
            Self::SubmitStore => "submit.store",
        }
    }

    /// Safe running message, when the source reports one for this step.
    pub const fn running_message(self) -> Option<&'static str> {
        match self {
            Self::SourcePrepare => Some("Preparing publication source"),
            Self::IngestionCreate => Some("Creating ingestion session"),
            Self::IngestionWait => Some("Waiting for portal ingestion to finish"),
            Self::SessionLoad => Some("Loading publication session"),
            Self::BundleLoad => Some("Loading publication bundle"),
            Self::CleanupRecover => Some("Recovering final publication state"),
            Self::CleanupRelease => Some("Rolling back failed publication release"),
            Self::MintPrepare => Some("Preparing release NFT transaction"),
            Self::MintSubmit => None,
            Self::MintSave => Some("Saving release NFT data"),
            Self::VerifyPrepare => Some("Preparing collection verification transaction"),
            Self::VerifySubmit => Some("Submitting collection verification transaction"),
            Self::AttestationCreate => Some("Creating attestation payload"),
            Self::SubmitStore => Some("Submitting release to store"),
        }
    }

    /// Safe completion message, when the source reports one for this step.
    ///
    /// Cleanup completion depends on the provider action and is represented by
    /// [`CleanupOutcome`] instead.
    pub const fn complete_message(self) -> Option<&'static str> {
        match self {
            Self::IngestionCreate => Some("Ingestion session created"),
            Self::IngestionWait => Some("Portal ingestion is ready"),
            Self::SessionLoad => Some("Publication session loaded"),
            Self::BundleLoad => Some("Publication bundle loaded"),
            Self::CleanupRecover => Some("Recovered final publication state"),
            Self::MintSubmit => Some("Release NFT transaction submitted"),
            Self::MintSave => Some("Release NFT data saved"),
            Self::VerifySubmit => Some("Release collection verified"),
            Self::AttestationCreate => Some("Attestation payload created"),
            Self::SubmitStore => Some("Release submitted to store"),
            Self::SourcePrepare
            | Self::CleanupRelease
            | Self::MintPrepare
            | Self::VerifyPrepare => None,
        }
    }
}

/// Progress event status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressStatus {
    /// The step started.
    Running,
    /// The step completed.
    Complete,
}

impl ProgressStatus {
    /// Wire status name used by the pinned source logger.
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Complete => "complete",
        }
    }
}

/// One safe progress event that a caller may print or forward.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgressEvent {
    /// Step being reported.
    pub step: ProgressStep,
    /// Whether the step started or completed.
    pub status: ProgressStatus,
}

impl ProgressEvent {
    /// Creates a progress event.
    pub const fn new(step: ProgressStep, status: ProgressStatus) -> Self {
        Self { step, status }
    }

    /// Safe message for this event, when the source reports one.
    ///
    /// Messages are static text. They never include provider text, application
    /// metadata, file paths, URLs or uploaded bytes.
    pub const fn message(self) -> Option<&'static str> {
        match self.status {
            ProgressStatus::Running => self.step.running_message(),
            ProgressStatus::Complete => self.step.complete_message(),
        }
    }
}

/// Outcome of an explicit failed-release cleanup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupOutcome {
    /// The portal deleted the failed release.
    Deleted,
    /// The release was already submitted and is preserved.
    PreservedSubmitted,
    /// The portal reported an action this crate does not recognize.
    Unrecognized,
}

impl CleanupOutcome {
    /// Classifies the exact reported cleanup action.
    pub fn from_action(action: &str) -> Self {
        match action {
            "deleted" => Self::Deleted,
            "preservedSubmitted" => Self::PreservedSubmitted,
            _ => Self::Unrecognized,
        }
    }

    /// Safe completion message for this outcome.
    pub const fn message(self) -> &'static str {
        match self {
            Self::Deleted => "Failed publication release cleaned up",
            Self::PreservedSubmitted => {
                "Publication already reached the submitted state; preserving release"
            }
            Self::Unrecognized => {
                "Failed publication release cleanup returned an unrecognized action"
            }
        }
    }
}

/// Safe ingestion progress derived without echoing provider detail or stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IngestionProgress {
    /// The portal supplied a nonblank processing detail; its text remains data.
    ReportedDetail,
    /// The portal supplied a nonblank processing stage; its text remains data.
    ReportedStage,
    /// The ingestion request was created.
    Created,
    /// The ingestion request is queued.
    Queued,
    /// The portal is processing the APK.
    Processing,
    /// The ingestion session is ready.
    Ready,
}

impl IngestionProgress {
    /// Derives safe progress from a normalized ingestion session.
    ///
    /// Precedence matches the source: nonblank processing detail, nonblank
    /// processing stage, then the exact status labels `created`, `queued`,
    /// `processing`, `Ready` and `ready`. Other statuses, including `Failed`,
    /// return `None`; callers inspect the session's explicit fields instead.
    pub fn from_session(session: &PublicationIngestionSession) -> Option<Self> {
        if session
            .processing_detail
            .as_deref()
            .is_some_and(|detail| !detail.trim().is_empty())
        {
            return Some(Self::ReportedDetail);
        }

        if session
            .processing_stage
            .as_deref()
            .is_some_and(|stage| !stage.trim().is_empty())
        {
            return Some(Self::ReportedStage);
        }

        match session.status.as_str() {
            "created" => Some(Self::Created),
            "queued" => Some(Self::Queued),
            "processing" => Some(Self::Processing),
            "Ready" | "ready" => Some(Self::Ready),
            _ => None,
        }
    }

    /// Safe message for this progress state.
    ///
    /// Provider detail and stage are represented by a fixed running message so
    /// untrusted portal text is never promoted into console output.
    pub const fn message(self) -> &'static str {
        match self {
            Self::ReportedDetail | Self::ReportedStage => "Portal ingestion is running",
            Self::Created => "Portal ingestion request created",
            Self::Queued => "Portal ingestion queued",
            Self::Processing => "Portal ingestion is processing the APK",
            Self::Ready => "Portal ingestion is ready",
        }
    }
}

impl PublicationIngestionSession {
    /// Safe progress state for this session, when the source reports one.
    pub fn progress(&self) -> Option<IngestionProgress> {
        IngestionProgress::from_session(self)
    }
}
