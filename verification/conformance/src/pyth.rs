//! Execute public Pyth wire and account APIs in every maintained portability host.
use crate::{CaseResult, case};
use solana_kit_core::codecs::{Error, Limits};
use solana_kit_pyth::{
    DEFAULT_REDUCED_GUARDIAN_SET_SIZE, EMITTER_CHAIN_PYTHNET, PythPriceStatus,
    PythVerificationLevel, decode_price_update_v2_account, decode_pyth_price_account,
    get_guardian_set_index, get_vaa_signature_count, parse_accumulator_update_data,
    parse_pyth_price_feed_message, parse_pyth_wormhole_message, parse_wormhole_vaa,
    trim_vaa_signatures,
};

const DATA: &[u8] =
    include_bytes!("../../../crates/solana_kit_pyth/tests/fixtures/accumulator.bin");
const ITEM_LIMIT: usize = 64;
const GUARDIANS: usize = 13;

const LEGACY: &[u8] =
    include_bytes!("../../../crates/solana_kit_pyth/tests/fixtures/legacy-price.bin");
const FULL: &[u8] =
    include_bytes!("../../../crates/solana_kit_pyth/tests/fixtures/price-update-full.bin");
const PARTIAL: &[u8] =
    include_bytes!("../../../crates/solana_kit_pyth/tests/fixtures/price-update-partial.bin");
const STALE_CONTEXT_SLOT: u64 = 1026;
const VERIFICATION_OFFSET: usize = 40;
const FIXTURE_EMA_PRICE: i64 = 555;
const FIXTURE_EMA_CONFIDENCE_NUMERATOR: i64 = 77;
const FIXTURE_SIGNATURES: u8 = 5;

const FIXTURE_SLOT: u64 = 208_075_477;
const FIXTURE_RING_SIZE: u32 = 10_000;
const FIXTURE_PRICE: i64 = 960_310_184;
const FIXTURE_CONFIDENCE: u64 = 11_022_234;
const FIXTURE_EXPONENT: i32 = -8;

pub(crate) fn cases() -> [CaseResult; 6] {
    [
        case("pyth-upstream-wire-composition", composition()),
        case("pyth-preserved-signature-reduction", signatures()),
        case("pyth-bounded-forward-compatible-wire", boundaries()),
        case("pyth-legacy-account-fields-and-staleness", legacy_account()),
        case(
            "pyth-receiver-borsh-verification-variants",
            receiver_accounts(),
        ),
        case("pyth-account-limits-and-truncation", account_boundaries()),
    ]
}

fn limits(bytes: &[u8]) -> Limits {
    Limits::new(bytes.len(), ITEM_LIMIT)
}

/// Compare each nested frame with the independent pinned reference report.
/// Decoding and comparing these fields does not authenticate the commitment.
fn composition() -> Result<(), &'static str> {
    let update =
        parse_accumulator_update_data(DATA, limits(DATA)).map_err(|_| "Pyth accumulator")?;
    let vaa =
        parse_wormhole_vaa(update.vaa, limits(update.vaa)).map_err(|_| "Wormhole envelope")?;
    let root =
        parse_pyth_wormhole_message(vaa.payload, limits(vaa.payload)).map_err(|_| "AUWV root")?;

    let message = update
        .updates
        .first()
        .ok_or("Missing price update")?
        .message;
    let price =
        parse_pyth_price_feed_message(message, limits(message)).map_err(|_| "Price message")?;

    if (
        update.updates.len(),
        vaa.signatures.len(),
        vaa.emitter_chain,
    ) != (1, GUARDIANS, EMITTER_CHAIN_PYTHNET)
    {
        return Err("Pyth envelope metadata differs");
    }

    if (root.merkle_root.slot, root.merkle_root.ring_size) != (FIXTURE_SLOT, FIXTURE_RING_SIZE) {
        return Err("AUWV commitment metadata differs");
    }

    if (price.price, price.confidence, price.exponent)
        != (FIXTURE_PRICE, FIXTURE_CONFIDENCE, FIXTURE_EXPONENT)
    {
        return Err("Exact price components differ");
    }

    let feed = price.feed_id_hex().map_err(|_| "Feed identifier text")?;

    if feed != "879551021853eec7a7dc827578e8e69da7e4fa8148339aa0d3d5296405be4b1a" {
        return Err("Feed identifier differs");
    }

    Ok(())
}

fn signatures() -> Result<(), &'static str> {
    let update =
        parse_accumulator_update_data(DATA, limits(DATA)).map_err(|_| "Pyth accumulator")?;
    let original =
        parse_wormhole_vaa(update.vaa, limits(update.vaa)).map_err(|_| "Original VAA")?;
    let copied_bytes = Limits::new(update.vaa.len(), update.vaa.len());
    let trimmed = trim_vaa_signatures(update.vaa, DEFAULT_REDUCED_GUARDIAN_SET_SIZE, copied_bytes)
        .map_err(|_| "VAA signature reduction")?;
    let reduced = parse_wormhole_vaa(&trimmed, limits(&trimmed)).map_err(|_| "Reduced VAA")?;

    if get_guardian_set_index(&trimmed) != Ok(original.guardian_set_index)
        || get_vaa_signature_count(&trimmed) != Ok(DEFAULT_REDUCED_GUARDIAN_SET_SIZE)
        || reduced.payload != original.payload
        || reduced.sequence != original.sequence
    {
        return Err("Signature reduction changed the attested body");
    }

    if reduced.signatures != original.signatures[..usize::from(DEFAULT_REDUCED_GUARDIAN_SET_SIZE)] {
        return Err("Signature reduction changed guardian order");
    }

    Ok(())
}

fn boundaries() -> Result<(), &'static str> {
    let mut future = DATA.to_vec();
    future[5] = 1;
    future.extend_from_slice(&[99, 100]);
    let update =
        parse_accumulator_update_data(&future, limits(&future)).map_err(|_| "Future minor")?;

    if update.minor_version != 1 || update.extension != [99, 100] {
        return Err("Pyth extensions were discarded");
    }

    if parse_accumulator_update_data(DATA, Limits::new(DATA.len(), 0)) != Err(Error::LimitExceeded)
    {
        return Err("Pyth allocation limit was ignored");
    }

    future[7] = 1;

    if parse_accumulator_update_data(&future, limits(&future)) != Err(Error::InvalidDiscriminator) {
        return Err("Unknown Pyth proof variant was accepted");
    }

    Ok(())
}

/// Compare decoded fields and derived status with the pinned Dart fixture.
/// The stored trading status must survive the caller's stale read context.
fn legacy_account() -> Result<(), &'static str> {
    let account = decode_pyth_price_account(LEGACY, Some(STALE_CONTEXT_SLOT), limits(LEGACY))
        .map_err(|_| "Legacy price account")?;

    if account.status != PythPriceStatus::Unknown
        || account.aggregate.status != PythPriceStatus::Trading
        || account.price_components.len() != 2
        || account.ema_price.value_component != FIXTURE_EMA_PRICE
        || account.ema_confidence.numerator != FIXTURE_EMA_CONFIDENCE_NUMERATOR
    {
        return Err("Legacy fields or derived staleness differ");
    }

    if account.price_components[0].publisher.to_bytes() != [0xee; 32]
        || account.price_components[1].publisher.to_bytes() != [0xef; 32]
        || account.price_components[1].latest.price_component != 3
    {
        return Err("Legacy publisher order or samples differ");
    }

    Ok(())
}

fn receiver_accounts() -> Result<(), &'static str> {
    let full = decode_price_update_v2_account(FULL, Limits::new(FULL.len(), 0))
        .map_err(|_| "Compact full verification account")?;
    let partial = decode_price_update_v2_account(PARTIAL, Limits::new(PARTIAL.len(), 0))
        .map_err(|_| "Partial verification account")?;

    if full.price_message != partial.price_message
        || full.price_message.confidence != u64::MAX
        || full.price_message.ema_conf != u64::MAX
        || full.price_message.price != 3_000_123_456_789
        || full.price_message.ema_price != -2_999_000_000
        || full.verification_level != PythVerificationLevel::Full
        || partial.verification_level.num_signatures() != Some(FIXTURE_SIGNATURES)
    {
        return Err("Receiver verification variants or exact integer fields differ");
    }

    Ok(())
}

fn account_boundaries() -> Result<(), &'static str> {
    if decode_pyth_price_account(LEGACY, None, Limits::new(LEGACY.len(), 1))
        != Err(Error::LimitExceeded)
        || decode_price_update_v2_account(&FULL[..FULL.len() - 1], limits(FULL))
            != Err(Error::Truncated)
    {
        return Err("Account item limit or truncated prefix was accepted");
    }

    let mut invalid = PARTIAL.to_vec();
    invalid[VERIFICATION_OFFSET] = 2;

    if decode_price_update_v2_account(&invalid, limits(&invalid))
        != Err(Error::InvalidDiscriminator)
    {
        return Err("Unknown receiver verification variant was accepted");
    }

    Ok(())
}
