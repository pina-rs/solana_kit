//! A signature is causally gated by the matching real Approve button event.
use super::{Action, LocalWalletCommands, LocalWalletEvents, show};
use solana_kit_leptoskit::wallet_ui::{LocalSigningReview, local_signing_approval};

pub(super) enum Decision {
    Approve,
    Reject,
    Cancel,
}

/// Ignore stale button generations while keeping rejection distinct from
/// cancellation, which disposes the selected owner instead of reusing its key.
pub(super) async fn request(
    purpose: &str,
    message: &[u8],
    generation: u64,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Decision {
    let review = LocalSigningReview {
        generation,
        purpose,
        message,
    };
    show(
        local_signing_approval(&review, commands),
        "Awaiting your approval",
    );

    while let Ok(action) = actions.next_action().await {
        let decision = match action {
            Action::Approve(current) if current == generation => Decision::Approve,

            Action::Reject(current) if current == generation => Decision::Reject,
            Action::Cancel | Action::Disconnect => Decision::Cancel,
            _ => continue,
        };

        return decision;
    }

    Decision::Cancel
}
