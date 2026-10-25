use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};
use base64::{Engine, engine::general_purpose::STANDARD};
use core::{any::Any, fmt};

/// The implemented Wallet Standard contract version.
pub const WALLET_STANDARD_VERSION: &str = "1.0.0";
/// Canonical Solana chain identifiers, in reference order.
pub const SOLANA_CHAINS: [&str; 4] = [
    "solana:mainnet",
    "solana:devnet",
    "solana:testnet",
    "solana:localnet",
];

/// Whether an identifier has nonempty namespace and name components.
pub fn is_identifier(value: &str) -> bool {
    value
        .split_once(':')
        .is_some_and(|(namespace, name)| !namespace.is_empty() && !name.is_empty())
}

/// Metadata rejected by a boundary constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetadataError {
    /// An account's display address was empty.
    EmptyAddress,
    /// A chain or feature identifier was not namespaced.
    InvalidIdentifier(String),
    /// The icon URI or base64 payload was malformed.
    InvalidIcon,
}

impl fmt::Display for MetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl core::error::Error for MetadataError {}

/// An immutable validated base64 image data URI. Image contents are not decoded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletIcon {
    data_uri: String,
    mime_type: &'static str,
    bytes: Vec<u8>,
}

impl WalletIcon {
    /// Parses SVG, WebP, PNG, or GIF. Empty image data is accepted by the source.
    pub fn parse(data_uri: impl Into<String>) -> Result<Self, MetadataError> {
        let data_uri = data_uri.into();
        let (prefix, payload) = data_uri
            .split_once(";base64,")
            .ok_or(MetadataError::InvalidIcon)?;

        let mime_type = match prefix {
            "data:image/svg+xml" => "image/svg+xml",
            "data:image/webp" => "image/webp",
            "data:image/png" => "image/png",
            "data:image/gif" => "image/gif",
            _ => return Err(MetadataError::InvalidIcon),
        };

        let bytes = STANDARD
            .decode(payload)
            .map_err(|_| MetadataError::InvalidIcon)?;

        Ok(Self {
            data_uri,
            mime_type,
            bytes,
        })
    }

    /// The original data URI.
    pub fn data_uri(&self) -> &str {
        &self.data_uri
    }
    /// The complete image MIME type.
    pub fn mime_type(&self) -> &str {
        self.mime_type
    }
    /// The image MIME subtype.
    pub fn mime_subtype(&self) -> &str {
        &self.mime_type[6..]
    }
    /// Raw decoded image bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Display for WalletIcon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.data_uri)
    }
}

/// Immutable account metadata. The display address is independent of the key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WalletAccount {
    address: String,
    public_key: [u8; 32],
    chains: Vec<String>,
    features: Vec<String>,
    label: Option<String>,
    icon: Option<WalletIcon>,
}

impl WalletAccount {
    /// Validates identifiers and a nonempty display address. The key is 32 bytes
    /// by construction. No base58 or address/key equality check is imposed.
    pub fn new(
        address: String,
        public_key: [u8; 32],
        chains: Vec<String>,
        features: Vec<String>,
        label: Option<String>,
        icon: Option<WalletIcon>,
    ) -> Result<Self, MetadataError> {
        if address.is_empty() {
            return Err(MetadataError::EmptyAddress);
        }

        for identifier in chains.iter().chain(&features) {
            if !is_identifier(identifier) {
                return Err(MetadataError::InvalidIdentifier(identifier.clone()));
            }
        }

        Ok(Self {
            address,
            public_key,
            chains,
            features,
            label,
            icon,
        })
    }

    /// The wallet's display address.
    pub fn address(&self) -> &str {
        &self.address
    }
    /// The exact public key.
    pub fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }
    /// Supported chain identifiers.
    pub fn chains(&self) -> &[String] {
        &self.chains
    }
    /// Supported feature identifiers.
    pub fn features(&self) -> &[String] {
        &self.features
    }
    /// Optional display label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
    /// Optional icon.
    pub fn icon(&self) -> Option<&WalletIcon> {
        self.icon.as_ref()
    }
}

/// Versioned concrete feature metadata. Versions are preserved without guessing
/// whether a future version remains compatible.
pub trait WalletFeature: Any {
    /// The exact feature contract version advertised by the backend.
    fn version(&self) -> &str;
}

trait ErasedFeature {
    fn value(&self) -> &dyn Any;
    fn version(&self) -> &str;
}

impl<T: WalletFeature> ErasedFeature for T {
    fn value(&self) -> &dyn Any {
        self
    }
    fn version(&self) -> &str {
        WalletFeature::version(self)
    }
}

/// Extensible feature discovery retaining concrete backend types and futures.
#[derive(Clone, Default)]
pub struct WalletFeatures(BTreeMap<String, Rc<dyn ErasedFeature>>);

impl WalletFeatures {
    /// Creates an empty feature map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a capability under a validated identifier.
    pub fn insert<T: WalletFeature>(
        &mut self,
        identifier: impl Into<String>,
        feature: T,
    ) -> Result<(), MetadataError> {
        let identifier = identifier.into();

        if !is_identifier(&identifier) {
            return Err(MetadataError::InvalidIdentifier(identifier));
        }

        self.0.insert(identifier, Rc::new(feature));
        Ok(())
    }

    /// Retrieves a capability only when its concrete type matches.
    pub fn feature<T: WalletFeature>(&self, identifier: &str) -> Option<&T> {
        self.0.get(identifier)?.value().downcast_ref()
    }

    /// Whether an identifier is advertised, including unknown extensions.
    pub fn supports(&self, identifier: &str) -> bool {
        self.0.contains_key(identifier)
    }
    /// The exact version for a known or future feature.
    pub fn version(&self, identifier: &str) -> Option<&str> {
        self.0.get(identifier).map(|feature| feature.version())
    }
    /// All advertised identifiers in lexical order.
    pub fn identifiers(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }
}

/// Caller-implemented wallet. Accounts represent actual backend authorization.
pub trait Wallet {
    /// Implemented Wallet Standard version.
    fn version(&self) -> &str;
    /// Canonical display name used for registry deduplication.
    fn name(&self) -> &str;
    /// Canonical icon.
    fn icon(&self) -> &WalletIcon;
    /// Current owned chain snapshot, safe across backend metadata changes.
    fn chains(&self) -> Vec<String>;
    /// Current capability snapshot retaining each concrete feature owner.
    fn features(&self) -> WalletFeatures;
    /// Current owned account authorization snapshot.
    fn accounts(&self) -> Vec<WalletAccount>;
}
