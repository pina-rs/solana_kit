//! Closed provider string enums reject unknown wire values.

macro_rules! string_enum {
    ($(#[$attribute:meta])* $doc:literal, $name:ident {
        $($(#[$variant_attribute:meta])* $variant:ident => $wire:literal),+ $(,)?
    }) => {
        $(#[$attribute])*
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $($(#[$variant_attribute])*
            #[doc = concat!("The `", $wire, "` wire value.")]
            #[serde(rename = $wire)]
            $variant,)+
        }

        impl $name {
            /// Returns the provider's exact wire spelling.
            pub const fn as_str(self) -> &'static str {
                const VALUES: &[&str] = &[$($wire),+];

                // Unit variants have contiguous implicit discriminants. Both the
                // variants and this table come from the same ordered repetition.
                VALUES[self as usize]
            }
        }
    };
}

string_enum!(
    "The Helius cluster.",
    HeliusCluster {
        Mainnet => "mainnet-beta",
        Devnet => "devnet",
    }
);
string_enum!(
    "Enhanced transaction queries exclude the unsupported processed commitment.",
    EnhancedCommitment {
        Confirmed => "confirmed",
        Finalized => "finalized",
    }
);
string_enum!(
    "Ordering of a provider's transaction history.",
    SortOrder {
        Ascending => "asc",
        Descending => "desc",
    }
);
string_enum!(
    "An asset's program standard.",
    HeliusAssetInterface {
        V1Nft => "V1_NFT",
        Custom => "Custom",
        V1Print => "V1_PRINT",
        LegacyNft => "Legacy_NFT",
        V2Nft => "V2_NFT",
        FungibleAsset => "FungibleAsset",
        Identity => "Identity",
        Executable => "Executable",
        ProgrammableNft => "ProgrammableNFT",
        FungibleToken => "FungibleToken",
        MplCoreAsset => "MplCoreAsset",
        MplCoreCollection => "MplCoreCollection",
    }
);
string_enum!(
    "How an asset's ownership is modeled.",
    HeliusOwnershipModel {
        Single => "single",
        Token => "token",
    }
);
string_enum!(
    "The asset royalty distribution model.",
    HeliusRoyaltyModel {
        Creators => "creators",
        Fanout => "fanout",
        Single => "single",
    }
);
string_enum!(
    "An asset authority's permitted scope.",
    HeliusScope {
        Full => "full",
        Royalty => "royalty",
        Metadata => "metadata",
        Extension => "extension",
    }
);
string_enum!(
    "How asset uses are consumed.",
    HeliusUseMethod {
        Burn => "Burn",
        Single => "Single",
        Multiple => "Multiple",
    }
);
string_enum!(
    "The intended asset rendering context.",
    HeliusContext {
        WalletDefault => "wallet-default",
        WebDesktop => "web-desktop",
        WebMobile => "web-mobile",
        AppMobile => "app-mobile",
        AppDesktop => "app-desktop",
        App => "app",
        Vr => "vr",
    }
);
string_enum!(
    "The asset sorting field.",
    AssetSortBy {
        Id => "id",
        Created => "created",
        Updated => "updated",
        RecentAction => "recent_action",
        None => "none",
    }
);

string_enum!(
    "The asset sorting direction.",
    AssetSortDirection {
        Ascending => "asc",
        Descending => "desc",
    }
);
string_enum!(
    "The token standard classification.",
    TokenStandard {
        ProgrammableNonFungible => "ProgrammableNonFungible",
        NonFungible => "NonFungible",
        Fungible => "Fungible",
        FungibleAsset => "FungibleAsset",
        NonFungibleEdition => "NonFungibleEdition",
        UnknownStandard => "UnknownStandard",
    }
);
string_enum!(
    "The fee estimation priority level.",
    PriorityLevel {
        Min => "Min",
        Low => "Low",
        Medium => "Medium",
        High => "High",
        VeryHigh => "VeryHigh",
        UnsafeMax => "UnsafeMax",
        Default => "Default",
    }
);
string_enum!(
    "The requested transaction representation.",
    UiTransactionEncoding {
        Binary => "binary",
        Base64 => "base64",
        Base58 => "base58",
        Json => "json",
        JsonParsed => "jsonParsed",
    }
);
string_enum!(
    "The webhook payload format and cluster.",
    WebhookType {
        Enhanced => "enhanced",
        EnhancedDevnet => "enhancedDevnet",
        Raw => "raw",
        RawDevnet => "rawDevnet",
        Discord => "discord",
        DiscordDevnet => "discordDevnet",
    }
);
string_enum!(
    "The transaction delivery status filter.",
    TransactionStatus {
        All => "all",
        Success => "success",
        Failed => "failed",
    }
);
string_enum!(
    "The requested commitment level.",
    CommitmentLevel {
        Processed => "processed",
        Confirmed => "confirmed",
        Finalized => "finalized",
    }
);
string_enum!(
    #[derive(Default)]
    "The pinned Sender region key; endpoint policy remains caller-owned.",
    SenderRegion {
        #[default]
        Default => "Default",
        UsSlc => "US_SLC",
        UsEast => "US_EAST",
        EuWest => "EU_WEST",
        EuCentral => "EU_CENTRAL",
        EuNorth => "EU_NORTH",
        ApSingapore => "AP_SINGAPORE",
        ApTokyo => "AP_TOKYO",
    }
);

string_enum!("History record representation.", TransactionDetails {
    Signatures => "signatures", Full => "full",
});
string_enum!("Transaction encoding.", TransactionEncoding {
    Json => "json", JsonParsed => "jsonParsed", Base64 => "base64", Base58 => "base58",
});
string_enum!("Address history transaction status filter.", HistoryTransactionStatus {
    Succeeded => "succeeded", Failed => "failed", Any => "any",
});
string_enum!("Related token-account inclusion.", TokenAccountMode {
    None => "none", BalanceChanged => "balanceChanged", All => "all",
});
