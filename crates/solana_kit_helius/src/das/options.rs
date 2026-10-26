//! Provider option objects are nested under their documented request keys.

use crate::enums::{AssetSortBy, AssetSortDirection};

request!(
    #[serde(deny_unknown_fields)]
    "Additional fields available from one asset or an asset batch.",
    AssetOptions {} optional {
        show_fungible: bool => "showFungible",
        show_unverified_collections: bool => "showUnverifiedCollections",
        show_collection_metadata: bool => "showCollectionMetadata",
        show_inscription: bool => "showInscription",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Additional fields available from asset list and search operations.",
    DisplayOptions {} optional {
        show_unverified_collections: bool => "showUnverifiedCollections",
        show_collection_metadata: bool => "showCollectionMetadata",
        show_grand_total: bool => "showGrandTotal",
        show_inscription: bool => "showInscription",
        show_raw_data: bool => "showRawData",
        show_fungible: bool => "showFungible",
        require_full_index: bool => "requireFullIndex",
        show_system_metadata: bool => "showSystemMetadata",
        show_zero_balance: bool => "showZeroBalance",
        show_closed_accounts: bool => "showClosedAccounts",
        show_native_balance: bool => "showNativeBalance",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "A sorting field and direction inside the outer sortBy object.",
    AssetSortingRequest {
        sort_by: AssetSortBy => "sortBy",
        sort_direction: AssetSortDirection => "sortDirection",
    } optional {}
);
request!(
    #[serde(deny_unknown_fields)]
    "Token-account display options.",
    TokenAccountOptions {} optional { show_zero_balance: bool => "showZeroBalance" }
);
