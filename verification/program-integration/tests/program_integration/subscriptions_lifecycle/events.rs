//! Anchor-compatible event bytes emitted as self-CPI inner instructions.
//!
//! The pinned program emits events by invoking its own `EmitEvent`
//! instruction, so the event payload is the inner instruction data rather than
//! a `Program data:` log line. The confirmed `getTransaction` response keeps
//! that data, and the generated event codecs validate the full wire frame.

use solana_kit_client::{
    Presence,
    methods::transaction_results::{
        InnerInstruction, InnerInstructions, JsonTransaction, RawMessage, TransactionMeta,
        TransactionView,
    },
};
use solana_kit_core::{Signature, codecs::Decoder};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, fixtures::SUBSCRIPTIONS,
};
use solana_kit_subscriptions::{
    SubscriptionCancelledEvent, SubscriptionCancelledEventCodec, SubscriptionCreatedEvent,
    SubscriptionCreatedEventCodec, SubscriptionResumedEvent, SubscriptionResumedEventCodec,
};

// `sha256("anchor:event")[..8]`, then the program event discriminator.
const EVENT_TAG: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
const CREATED_EVENT_DISCRIMINATOR: u8 = 0;
const CANCELLED_EVENT_DISCRIMINATOR: u8 = 1;
const RESUMED_EVENT_DISCRIMINATOR: u8 = 5;

pub(crate) enum SubscriptionEvent {
    Created(SubscriptionCreatedEvent),
    Cancelled(SubscriptionCancelledEvent),
    Resumed(SubscriptionResumedEvent),
}

pub(crate) async fn lifecycle_event(
    env: &IntegrationEnv,
    signature: Signature,
) -> Result<SubscriptionEvent, Error> {
    let data = event_instruction_data(env, signature).await?;

    decode_event(data)
}

/// Reads the one event payload from a confirmed lifecycle transaction.
///
/// Every branch here is a deliberate refusal to guess: the request pins JSON
/// encoding so inner calls stay compiled, legacy compilation leaves the program
/// index unambiguous, and any count other than one means the event surface is
/// not the pinned program's single self-CPI. Rejecting those shapes keeps the
/// decoder from attributing bytes to the wrong instruction.
async fn event_instruction_data(
    env: &IntegrationEnv,
    signature: Signature,
) -> Result<Vec<u8>, Error> {
    let transaction = env
        .transaction(signature)
        .await?
        .ok_or(Error::Client("the lifecycle transaction is unavailable"))?;
    let meta = transaction
        .details
        .meta
        .ok_or(Error::Client("the lifecycle transaction has no metadata"))?;
    let TransactionView::Json(json) = transaction.details.transaction else {
        return Err(Error::Client(
            "the lifecycle transaction was not returned as JSON",
        ));
    };

    require_legacy_resolution(&json)?;
    let index = program_index(&json)?;
    let events = collect_program_data(index, inner_groups(meta)?)?;

    match events.len() {
        1 => Ok(events.into_iter().next().expect("one event was collected")),
        count => Err(Error::Program(format!(
            "the lifecycle transaction emitted {count} subscriptions inner instructions, expected one event"
        ))),
    }
}

fn require_legacy_resolution(json: &JsonTransaction<RawMessage>) -> Result<(), Error> {
    match &json.message.address_table_lookups {
        Presence::Present(lookups) if !lookups.is_empty() => Err(Error::Client(
            "loaded addresses would make the inner program index ambiguous",
        )),
        _ => Ok(()),
    }
}

fn program_index(json: &JsonTransaction<RawMessage>) -> Result<usize, Error> {
    let program = SUBSCRIPTIONS.address();

    json.message
        .account_keys
        .iter()
        .position(|key| *key == program)
        .ok_or(Error::Client(
            "the lifecycle transaction did not reference the subscriptions program",
        ))
}

fn inner_groups(meta: TransactionMeta) -> Result<Vec<InnerInstructions>, Error> {
    match meta.inner_instructions {
        Presence::Present(Some(groups)) => Ok(groups),
        _ => Err(Error::Client(
            "the lifecycle transaction has no inner instructions",
        )),
    }
}

fn collect_program_data(
    program_index: usize,
    groups: Vec<InnerInstructions>,
) -> Result<Vec<Vec<u8>>, Error> {
    let mut events = Vec::new();

    for instruction in groups.into_iter().flat_map(|group| group.instructions) {
        let InnerInstruction::Compiled(compiled) = instruction else {
            return Err(Error::Client(
                "the lifecycle transaction returned resolved inner instructions",
            ));
        };

        if usize::from(compiled.program_id_index) == program_index {
            events.push(compiled.data);
        }
    }

    Ok(events)
}

fn decode_event(data: Vec<u8>) -> Result<SubscriptionEvent, Error> {
    let tag = data.get(..EVENT_TAG.len());

    if tag != Some(EVENT_TAG.as_slice()) {
        return Err(Error::Program(
            "the subscriptions event does not start with the anchor event tag".to_owned(),
        ));
    }

    let discriminator = *data.get(EVENT_TAG.len()).ok_or(Error::Program(
        "the subscriptions event is missing its discriminator".to_owned(),
    ))?;

    match discriminator {
        CREATED_EVENT_DISCRIMINATOR => SubscriptionCreatedEventCodec
            .decode(&data, CODEC_LIMITS)
            .map(SubscriptionEvent::Created)
            .map_err(|error| Error::Program(format!("subscriptions created event: {error}"))),
        CANCELLED_EVENT_DISCRIMINATOR => SubscriptionCancelledEventCodec
            .decode(&data, CODEC_LIMITS)
            .map(SubscriptionEvent::Cancelled)
            .map_err(|error| Error::Program(format!("subscriptions cancelled event: {error}"))),
        RESUMED_EVENT_DISCRIMINATOR => SubscriptionResumedEventCodec
            .decode(&data, CODEC_LIMITS)
            .map(SubscriptionEvent::Resumed)
            .map_err(|error| Error::Program(format!("subscriptions resumed event: {error}"))),
        other => Err(Error::Program(format!(
            "unexpected subscriptions event discriminator {other}"
        ))),
    }
}
