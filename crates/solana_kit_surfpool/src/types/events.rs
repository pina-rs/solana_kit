//! Flat runtime event models, independent of any event source or clock.

object_model! {
/// Clock payload attached to runtime events.
#[derive(Default)]
pub struct ClockValue {
    /// Current slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<i128>,
    /// Epoch start Unix timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch_start_timestamp: Option<i128>,
    /// Current epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i128>,
    /// Leader schedule epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_schedule_epoch: Option<i128>,
    /// Current Unix timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unix_timestamp: Option<i128>,
}
}

object_model! {
/// Flat event record matching every declared pinned Dart field.
pub struct SimnetEventValue {
    /// Runtime event discriminator.
    pub kind: String,
    /// Human-readable message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Runtime-provided event timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Initial transaction count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_transaction_count: Option<i128>,
    /// Clock payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<ClockValue>,
    /// Epoch payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch_info: Option<super::EpochInfoValue>,
    /// Clock command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock_command: Option<String>,
    /// Slot interval in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot_interval_ms: Option<i128>,
    /// Updated account public key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_pubkey: Option<String>,
    /// Transaction signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_signature: Option<String>,
    /// Transaction or profile logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    /// Consumed compute units.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_units_consumed: Option<i128>,
    /// Transaction fee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<i128>,
    /// Runtime error text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Profile tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Profile key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_key: Option<String>,
    /// Profile slot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_slot: Option<i128>,
    /// Runbook identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook_id: Option<String>,
    /// Runbook failures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook_errors: Option<Vec<String>>,
}
}
