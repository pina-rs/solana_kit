use serde_json::Number;
use std::{fmt, string::String, vec::Vec};

/// dApp identity, display metadata and publisher-supplied contact fields.
///
/// Every value is untrusted backend text. Contact addresses and URLs are for
/// explicit caller review; this model never transmits or fetches them.
#[derive(Clone, PartialEq)]
pub struct DappInfo {
    /// Portal dApp identity.
    pub id: String,
    /// Display name, falling back to the normalized release name.
    pub dapp_name: String,
    /// Optional subtitle, including an explicit blank value.
    pub subtitle: Option<String>,
    /// Long description, empty when the backend reported no string.
    pub description: String,
    /// Android package identifier from the dApp, then release record.
    pub android_package: String,
    /// Optional icon location.
    pub dapp_icon_url: Option<String>,
    /// Preview locations in provider order.
    pub dapp_preview_urls: Vec<String>,
    /// Optional banner location.
    pub banner_url: Option<String>,
    /// Optional feature-graphic location.
    pub feature_graphic_url: Option<String>,
    /// Optional editors'-choice graphic location.
    pub editors_choice_graphic_url: Option<String>,
    /// Optional application website.
    pub app_website: Option<String>,
    /// Optional publisher contact address.
    pub contact_email: Option<String>,
    /// Support address from the dApp support field, then publisher support.
    pub support_email: String,
    /// Language identifiers in provider order.
    pub languages: Vec<String>,
    /// Optional licence location.
    pub license_url: Option<String>,
    /// Optional copyright location.
    pub copyright_url: Option<String>,
    /// Optional privacy-policy location.
    pub privacy_policy_url: Option<String>,
    /// Proposed dApp wallet text; no address validation is performed here.
    pub wallet_address: String,
    /// Proposed dApp collection mint text.
    pub nft_mint_address: String,
    /// Previous approved release identity.
    pub last_approved_release_id: Option<String>,
    /// Website from the dApp `website` field, then `appWebsite`.
    pub website: Option<String>,
}

/// The publisher kind reported by the portal.
#[derive(Clone, Eq, PartialEq)]
pub enum PublisherKind {
    /// An organization; also the source default when no type is reported.
    Organization,
    /// An individual publisher.
    Individual,
    /// A reported kind that is neither `organization` nor `individual`.
    ///
    /// The pinned source coerces this to `organization`. Rust retains the
    /// reported value so a caller cannot mistake an unknown kind for a verified
    /// organization.
    Unrecognized(String),
}

impl fmt::Debug for PublisherKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Organization => formatter.write_str("Organization"),
            Self::Individual => formatter.write_str("Individual"),
            Self::Unrecognized(_) => formatter.write_str("Unrecognized"),
        }
    }
}

impl PublisherKind {
    /// Maps an optional backend `type` string.
    ///
    /// Absent and explicit null values become [`Self::Organization`], matching
    /// the source. Unknown strings become [`Self::Unrecognized`] instead of
    /// being presented as a verified organization.
    pub fn from_reported(value: Option<&str>) -> Self {
        match value {
            None | Some("organization") => Self::Organization,
            Some("individual") => Self::Individual,
            Some(other) => Self::Unrecognized(other.to_owned()),
        }
    }
}

/// Publisher identity and contact fields from the portal.
#[derive(Clone, PartialEq)]
pub struct PublisherInfo {
    /// Portal publisher identity.
    pub id: String,
    /// Reported publisher kind.
    pub kind: PublisherKind,
    /// Publisher display name.
    pub name: String,
    /// Publisher website.
    pub website: String,
    /// Publisher contact address.
    pub email: String,
    /// Support address from the publisher, dApp, then publisher contact field.
    pub support_email: String,
}

/// Legal links attached to a publication bundle.
#[derive(Clone, PartialEq)]
pub struct BundleLegal {
    /// Optional licence location.
    pub license_url: Option<String>,
    /// Optional copyright location.
    pub copyright_url: Option<String>,
    /// Optional privacy-policy location.
    pub privacy_policy_url: Option<String>,
}

/// One localized release string resource.
#[derive(Clone, PartialEq)]
pub struct LocalizedReleaseStrings {
    /// Locale tag; the translator currently emits only `en-US`.
    pub locale: String,
    /// Localized release name.
    pub name: String,
    /// Localized short description.
    pub short_description: String,
    /// Localized long description.
    pub long_description: String,
    /// Localized release notes.
    pub new_in_version: String,
}

/// One media location declared on the dApp record.
///
/// The pinned translator leaves its untyped media list empty because media is
/// resolved later. Rust projects the declared locations without fetching them,
/// so callers can inspect the exact source set before any network work.
#[derive(Clone, PartialEq)]
pub struct BundleMedia {
    /// Deterministic media purpose.
    pub purpose: MediaPurpose,
    /// Exact declared location, without URL policy approval.
    pub uri: String,
}

/// Media purpose in the pinned metadata document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaPurpose {
    /// The required application icon.
    Icon,
    /// A preview screenshot.
    Screenshot,
    /// An optional banner.
    Banner,
    /// An optional feature graphic, preferring the editors'-choice field.
    FeatureGraphic,
}

impl MediaPurpose {
    /// Wire purpose used by the metadata document.
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Icon => "icon",
            Self::Screenshot => "screenshot",
            Self::Banner => "banner",
            Self::FeatureGraphic => "featureGraphic",
        }
    }
}

/// Normalized metadata for a publication bundle.
#[derive(Clone, PartialEq)]
pub struct BundleMetadata {
    /// Localized release name.
    pub localized_name: String,
    /// Short description from the release field, dApp subtitle, then a bounded description.
    pub short_description: String,
    /// Long description.
    pub long_description: String,
    /// Release notes; empty when the backend reported none.
    pub new_in_version: String,
    /// Optional publisher website, without a dApp fallback.
    pub publisher_website: Option<String>,
    /// Optional publisher support address, without a dApp fallback.
    pub support_email: Option<String>,
    /// Optional dApp application website, without a publisher fallback.
    pub website: Option<String>,
    /// dApp languages in provider order.
    pub locales: Vec<String>,
    /// Legal links.
    pub legal: BundleLegal,
    /// Declared media locations in deterministic icon, preview, banner, feature order.
    pub media: Vec<BundleMedia>,
    /// Install-file details for this bundle.
    pub install_file: InstallFileDetails,
    /// Localized resources resolved by the translator.
    pub localized_strings: Vec<LocalizedReleaseStrings>,
    /// Optional release metadata location.
    pub release_metadata_uri: Option<String>,
}

/// Signer authorities proposed by the portal record.
///
/// These are identifiers for caller review, not proof that a key controls the
/// address or that the same signer is acceptable for every transaction.
#[derive(Clone, PartialEq)]
pub struct SignerAuthority {
    /// dApp wallet address after the pinned fallback chain.
    pub dapp_wallet_address: String,
    /// Collection authority after the pinned fallback chain.
    pub collection_authority: String,
    /// Collection mint after the pinned fallback chain.
    pub app_mint_address: String,
    /// Whether the backend requires one signer; absent and null become true.
    pub same_signer_required: bool,
    /// Accepted roles; absent or null becomes the two defaults, while a reported array is retained.
    pub accepted_signer_roles: Vec<String>,
    /// dApp identity.
    pub dapp_id: String,
    /// Required signer, falling back to the resolved dApp wallet.
    pub required_signer: String,
    /// Mint signer, falling back to the resolved dApp wallet.
    pub mint_signer: String,
    /// Optional fee payer.
    pub fee_payer: Option<String>,
}

/// Where an install file is hosted.
#[derive(Clone, Eq, PartialEq)]
pub enum PortalSourceKind {
    /// Portal-managed storage, also the source inference for existing releases.
    PortalUpload,
    /// An externally hosted HTTPS location.
    ExternalUrl,
    /// A reported kind this crate does not recognize.
    Unrecognized(String),
}

impl fmt::Debug for PortalSourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PortalUpload => formatter.write_str("PortalUpload"),
            Self::ExternalUrl => formatter.write_str("ExternalUrl"),
            Self::Unrecognized(_) => formatter.write_str("Unrecognized"),
        }
    }
}

/// On-disk or remote install file projected from a backend bundle.
#[derive(Clone, PartialEq)]
pub struct InstallFileDetails {
    /// Exact install-file location, empty when neither record supplied one.
    pub uri: String,
    /// Media type supplied by the backend or inferred from the file name.
    pub mime_type: String,
    /// Exact reported size, zero when absent.
    pub size: Number,
    /// Optional provider hash text; no digest is verified here.
    pub sha256: Option<String>,
    /// File name from the release record, then the URI path.
    pub file_name: Option<String>,
    /// Canonical location, falling back to [`Self::uri`].
    pub canonical_url: String,
    /// Hosting kind inferred from the ingestion source.
    pub origin: PortalSourceKind,
}

impl InstallFileDetails {
    /// Source alias of [`Self::uri`].
    pub fn url(&self) -> &str {
        &self.uri
    }
}

/// Normalized release record from the portal.
#[derive(Clone, PartialEq)]
pub struct ReleaseInfo {
    /// Release identity.
    pub id: String,
    /// Linked dApp identity.
    pub dapp_id: String,
    /// Release file location after install-record fallback.
    pub release_file_url: String,
    /// Release file name after install-record fallback.
    pub release_file_name: String,
    /// Exact reported file size after install-record fallback.
    pub release_file_size: Number,
    /// Optional provider file hash.
    pub release_file_hash: Option<String>,
    /// Release display name after the pinned fallback chain.
    pub release_name: String,
    /// Version name, falling back to the exact version-code text or `1`.
    pub version_name: String,
    /// Exact reported version code, one when absent.
    pub version_code: Number,
    /// Android package identifier.
    pub android_package: String,
    /// Optional exact minimum SDK number.
    pub min_sdk_version: Option<Number>,
    /// Optional exact target SDK number.
    pub target_sdk_version: Option<Number>,
    /// Android permissions in provider order.
    pub permissions: Vec<String>,
    /// Release locales in provider order.
    pub locales: Vec<String>,
    /// Optional certificate fingerprint.
    pub certificate_fingerprint: Option<String>,
    /// Optional short description.
    pub short_description: Option<String>,
    /// Optional long description.
    pub long_description: Option<String>,
    /// Localized display name.
    pub localized_name: String,
    /// Release notes.
    pub new_in_version: String,
    /// Optional platform-feature text.
    pub saga_features: Option<String>,
    /// Optional proposed release mint text.
    pub release_mint_address: Option<String>,
    /// Optional release metadata location.
    pub release_metadata_uri: Option<String>,
    /// Optional NFT metadata location, kept distinct from the release alias.
    pub nft_metadata_uri: Option<String>,
}

impl ReleaseInfo {
    /// Source alias of [`Self::release_mint_address`].
    pub fn nft_mint_address(&self) -> Option<&str> {
        self.release_mint_address.as_deref()
    }
}

/// A normalized publication bundle.
#[derive(Clone, PartialEq)]
pub struct PublicationBundle {
    /// Ingestion session identity.
    pub ingestion_session_id: String,
    /// Publication session identity.
    pub publication_session_id: String,
    /// Release identity after the source fallback chain.
    pub release_id: String,
    /// dApp record.
    pub dapp: DappInfo,
    /// Publisher record.
    pub publisher: PublisherInfo,
    /// Install-file details.
    pub install_file: InstallFileDetails,
    /// Normalized metadata.
    pub metadata: BundleMetadata,
    /// Proposed signer authorities.
    pub signer_authority: SignerAuthority,
    /// Normalized release record.
    pub release: ReleaseInfo,
}

/// One precise publication lifecycle checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicationCheckpoint {
    /// The session exists without a prepared bundle.
    Created,
    /// The bundle is ready for caller-owned mint preparation.
    BundleReady,
    /// A mint submission was recorded.
    MintSubmitted,
    /// Release NFT data was saved.
    MintSaved,
    /// A collection-verification submission was recorded.
    VerificationSubmitted,
    /// Collection verification was recorded.
    Verified,
    /// An attestation was recorded.
    Attested,
    /// Store submission was recorded.
    Submitted,
    /// Publication completed.
    Completed,
}

impl PublicationCheckpoint {
    /// Portal wire name for this checkpoint.
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::BundleReady => "bundle-ready",
            Self::MintSubmitted => "mint-submitted",
            Self::MintSaved => "mint-saved",
            Self::VerificationSubmitted => "verification-submitted",
            Self::Verified => "verified",
            Self::Attested => "attested",
            Self::Submitted => "submitted",
            Self::Completed => "completed",
        }
    }

    /// Parses a wire checkpoint, defaulting unknown names to [`Self::Created`].
    ///
    /// The pinned source uses the same permissive default. The session
    /// translator does not use this method; it derives checkpoints from the
    /// precise stage/receipt evidence instead.
    pub fn from_wire(value: &str) -> Self {
        match value {
            "created" => Self::Created,
            "bundle-ready" => Self::BundleReady,
            "mint-submitted" => Self::MintSubmitted,
            "mint-saved" => Self::MintSaved,
            "verification-submitted" => Self::VerificationSubmitted,
            "verified" => Self::Verified,
            "attested" => Self::Attested,
            "submitted" => Self::Submitted,
            "completed" => Self::Completed,
            _ => Self::Created,
        }
    }

    /// Whether this checkpoint is at or beyond `expected` in the source order.
    pub const fn is_at_least(self, expected: Self) -> bool {
        self.rank() >= expected.rank()
    }

    const fn rank(self) -> u8 {
        match self {
            Self::Created => 0,
            Self::BundleReady => 1,
            Self::MintSubmitted => 2,
            Self::MintSaved => 3,
            Self::VerificationSubmitted => 4,
            Self::Verified => 5,
            Self::Attested => 6,
            Self::Submitted => 7,
            Self::Completed => 8,
        }
    }
}

/// The coarse stage reported by a publication session.
#[derive(Clone, Eq, PartialEq)]
pub enum PublicationSessionStage {
    /// Ready for caller-owned mint preparation.
    PreparedForMint,
    /// Mint submission was recorded.
    MintSubmitted,
    /// Mint data was saved.
    MintSaved,
    /// Collection-verification submission was recorded.
    VerificationSubmitted,
    /// Collection verification was recorded.
    Verified,
    /// An attestation was recorded.
    Attested,
    /// Store submission was recorded.
    Submitted,
    /// The publication failed.
    Failed,
    /// A reported stage name this crate does not recognize.
    ///
    /// The pinned translator silently maps unknown stages to
    /// [`Self::PreparedForMint`]. Rust retains the reported name so callers
    /// cannot mistake an unknown future stage for mint readiness.
    Unknown(String),
}

impl PublicationSessionStage {
    /// Maps an optional backend stage.
    ///
    /// Absent and explicit null values become [`Self::PreparedForMint`],
    /// matching the source. Blank and unknown names become [`Self::Unknown`].
    pub fn from_wire(value: Option<&str>) -> Self {
        match value {
            None | Some("PreparedForMint") => Self::PreparedForMint,
            Some("MintSubmitted") => Self::MintSubmitted,
            Some("MintSaved") => Self::MintSaved,
            Some("VerificationSubmitted") => Self::VerificationSubmitted,
            Some("Verified") => Self::Verified,
            Some("Attested") => Self::Attested,
            Some("Submitted") => Self::Submitted,
            Some("Failed") => Self::Failed,
            Some(other) => Self::Unknown(other.to_owned()),
        }
    }

    /// Reported stage text; unknown text is retained exactly.
    ///
    /// Unknown text is untrusted provider data and should not be logged.
    pub fn wire_name(&self) -> &str {
        match self {
            Self::PreparedForMint => "PreparedForMint",
            Self::MintSubmitted => "MintSubmitted",
            Self::MintSaved => "MintSaved",
            Self::VerificationSubmitted => "VerificationSubmitted",
            Self::Verified => "Verified",
            Self::Attested => "Attested",
            Self::Submitted => "Submitted",
            Self::Failed => "Failed",
            Self::Unknown(value) => value,
        }
    }
}

impl fmt::Debug for PublicationSessionStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreparedForMint => formatter.write_str("PreparedForMint"),
            Self::MintSubmitted => formatter.write_str("MintSubmitted"),
            Self::MintSaved => formatter.write_str("MintSaved"),
            Self::VerificationSubmitted => formatter.write_str("VerificationSubmitted"),
            Self::Verified => formatter.write_str("Verified"),
            Self::Attested => formatter.write_str("Attested"),
            Self::Submitted => formatter.write_str("Submitted"),
            Self::Failed => formatter.write_str("Failed"),
            Self::Unknown(_) => formatter.write_str("Unknown"),
        }
    }
}

/// Session status reported for a publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicationSessionStatus {
    /// The session is pending.
    Pending,
    /// The session is running.
    Running,
    /// The session completed.
    Completed,
    /// The session failed.
    Failed,
}

impl PublicationSessionStatus {
    /// Maps an optional backend status.
    ///
    /// Unknown, absent and null values become [`Self::Running`], matching the
    /// source default. The publication-session translator derives status from
    /// the stage and ticket evidence rather than this method.
    pub fn from_wire(value: Option<&str>) -> Self {
        match value {
            Some("pending") => Self::Pending,
            Some("completed") => Self::Completed,
            Some("failed") => Self::Failed,
            _ => Self::Running,
        }
    }
}

/// Caller-observed timestamps used only where a backend session omits a value.
///
/// The pinned translator samples the wall clock for absent timestamps. Rust
/// requires the caller to supply any fallback explicitly; without one the
/// normalized timestamp remains absent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTimestamps {
    /// Fallback creation timestamp.
    pub created: String,
    /// Fallback update timestamp.
    pub updated: String,
}

/// A normalized publication session.
#[derive(Clone, PartialEq)]
pub struct PublicationSession {
    /// Portal publication-session identity.
    pub id: String,
    /// Ingestion-session identity.
    pub ingestion_session_id: String,
    /// Release identity.
    pub release_id: String,
    /// Derived session status.
    pub status: PublicationSessionStatus,
    /// Reported coarse stage.
    pub stage: PublicationSessionStage,
    /// Derived precise checkpoint.
    pub checkpoint: PublicationCheckpoint,
    /// Optional metadata location.
    pub metadata_uri: Option<String>,
    /// Optional expected release mint text.
    pub release_mint_address: Option<String>,
    /// Optional mint-submission signature text.
    pub mint_transaction_signature: Option<String>,
    /// Optional collection-verification signature text.
    pub verify_transaction_signature: Option<String>,
    /// Optional attestation request identity.
    pub attestation_request_unique_id: Option<String>,
    /// Optional store-review ticket.
    pub hubspot_ticket_id: Option<String>,
    /// Optional untrusted backend error text.
    ///
    /// This is the source's normalized-error slot. It contains exactly the same
    /// provider text as `last_error`; never include it in ordinary diagnostics.
    pub last_error: Option<String>,
    /// Optional creation timestamp, exact backend text or caller fallback.
    pub created: Option<String>,
    /// Optional update timestamp, exact backend text or caller fallback.
    pub updated: Option<String>,
}

impl PublicationSession {
    /// Source normalized-error alias of [`Self::last_error`].
    ///
    /// The pinned translator assigns the raw backend `lastError` to both model
    /// fields. Rust stores it once; callers that need the source alias can use
    /// this accessor. The text remains untrusted provider data.
    pub fn error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }
}

impl fmt::Debug for PublicationSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PublicationSession")
            .field("status", &self.status)
            .field("stage", &self.stage)
            .field("checkpoint", &self.checkpoint)
            .finish_non_exhaustive()
    }
}

/// A normalized ingestion session returned by the portal.
#[derive(Clone, PartialEq)]
pub struct PublicationIngestionSession {
    /// Ingestion-session identity.
    pub id: String,
    /// dApp identity.
    pub dapp_id: String,
    /// Caller-issued idempotency key reported by the portal.
    pub idempotency_key: String,
    /// Normalized source record.
    pub source: PublicationSource,
    /// Release notes.
    pub whats_new: String,
    /// Reported status, defaulting to the reported source kind.
    pub status: String,
    /// Optional release identity.
    pub release_id: Option<String>,
    /// Optional publication-session identity.
    pub publication_session_id: Option<String>,
    /// Optional attached bundle.
    pub bundle: Option<PublicationBundle>,
    /// Optional attached publication session.
    pub publication_session: Option<PublicationSession>,
    /// Optional Android package identifier.
    pub android_package: Option<String>,
    /// Optional release version name.
    pub version_name: Option<String>,
    /// Optional exact processing-progress number.
    pub processing_progress: Option<Number>,
    /// Optional untrusted processing stage text.
    pub processing_stage: Option<String>,
    /// Optional untrusted processing detail text.
    pub processing_detail: Option<String>,
    /// Optional untrusted processing error text.
    pub processing_error: Option<String>,
    /// Optional untrusted session error text.
    pub error: Option<String>,
}

impl PublicationIngestionSession {
    /// Whether the exact status is `Ready` or `ready`.
    pub fn is_ready(&self) -> bool {
        self.status == "Ready" || self.status == "ready"
    }

    /// Whether the exact status is `Failed` or `failed`.
    pub fn is_failed(&self) -> bool {
        self.status == "Failed" || self.status == "failed"
    }
}

impl fmt::Debug for PublicationIngestionSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PublicationIngestionSession")
            .field("ready", &self.is_ready())
            .field("failed", &self.is_failed())
            .finish_non_exhaustive()
    }
}

/// The normalized source of an ingested APK.
///
/// A local file is a caller input (`workflow::LocalApk` or the CLI source enum)
/// and never a backend projection. The translated variants carry only fields the
/// pinned translator can establish; upload hashes and content-type overrides stay
/// request-side.
#[derive(Clone, PartialEq)]
pub enum PublicationSource {
    /// APK already hosted on portal storage.
    PortalUpload {
        /// Exact reported location.
        release_file_url: String,
        /// Exact reported or inferred file name, possibly blank.
        release_file_name: String,
        /// Exact reported size, zero when absent.
        release_file_size: Number,
    },
    /// Caller-approved externally hosted APK.
    ExternalUrl {
        /// Exact reported URL.
        url: String,
        /// Optional file name from the session or URL path.
        file_name: Option<String>,
    },
    /// A prior release reused as the source.
    ExistingRelease {
        /// Existing release identity, possibly blank in the source record.
        source_release_id: String,
    },
    /// A reported source kind this crate does not recognize.
    ///
    /// The pinned translator would present this as a portal upload. Rust keeps
    /// the reported kind and exact fields so callers cannot upload against an
    /// unknown contract by accident.
    Unrecognized {
        /// Exact reported source-kind text.
        reported_kind: String,
        /// Exact reported source location.
        source_url: String,
        /// Optional reported or inferred file name.
        file_name: Option<String>,
        /// Exact reported size, zero when absent.
        release_file_size: Number,
    },
}
