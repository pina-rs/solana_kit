//! Open metadata values remain open only where the source schema permits them.

use std::collections::BTreeMap;

model!(
    "Digital asset content.",
    AssetContent {} optional {
        schema: String => "$schema",
        json_uri: String => "json_uri",
        files: Vec<AssetFile> => "files",
        metadata: AssetMetadata => "metadata",
        links: BTreeMap<String,
        serde_json::Value> => "links",
    }
);
model!(
    "An asset media file.",
    AssetFile {} optional {
        uri: String => "uri",
        cdn_uri: String => "cdn_uri",
        mime: String => "mime",
        quality: AssetFileQuality => "quality",
        contexts: Vec<String> => "contexts",
    }
);
model!(
    "An asset's display metadata.",
    AssetMetadata {} optional {
        name: String => "name",
        symbol: String => "symbol",
        description: String => "description",
        attributes: Vec<AssetAttribute> => "attributes",
        token_standard: String => "token_standard",
    }
);
model!(
    "A media file's quality schema.",
    AssetFileQuality { schema: String => "schema" } optional {}
);
model!(
    "Collection display metadata returned when requested.",
    AssetCollectionMetadata {} optional {
        name: String => "name",
        symbol: String => "symbol",
        image: String => "image",
        description: String => "description",
        external_url: String => "external_url",
    }
);
model!(
    "On-chain inscription facts; returned hashes are not authenticated here.",
    AssetInscription {} optional {
        order: u64 => "order",
        size: u64 => "size",
        content_type: String => "contentType",
        encoding: String => "encoding",
        validation_hash: String => "validationHash",
        inscription_data_account: String => "inscriptionDataAccount",
        authority: String => "authority",
    }
);
model!(
    "A metadata attribute; its source value is intentionally arbitrary JSON.",
    AssetAttribute {}
    optional { value: serde_json::Value => "value",
    trait_type: String => "trait_type" }
);
