//! Selected local keys are never automatically saved or exported.
use super::{
    Action, LocalWalletCommands, LocalWalletEvents,
    approval::{self, Decision},
    cancellable, show,
};
use solana_kit_core::crypto::verify;
use solana_kit_leptoskit::{GuestLocalWallet, WalletSessionError, wallet_ui::local_wallet_panel};

pub(super) async fn save(
    wallet: &mut GuestLocalWallet,
    action: Action,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Option<Result<&'static str, WalletSessionError>> {
    show(
        local_wallet_panel(Some(&wallet.address().to_string()), true, false, commands),
        "Saving private wallet data",
    );

    let (result, status) = match action {
        Action::Save => (
            cancellable(wallet.save(), actions).await,
            "Wallet saved privately",
        ),
        Action::Backup => (
            cancellable(wallet.backup(), actions).await,
            "Private copy created inside the app",
        ),
        _ => return Some(Ok("Local wallet connected")),
    };

    result.map(|result| result.map(|_| status))
}

pub(super) async fn sign(
    wallet: GuestLocalWallet,
    generation: u64,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<(GuestLocalWallet, &'static str), WalletSessionError> {
    let message = format!(
        "Solana Kit generated app: account {}, review {generation}",
        wallet.address()
    );

    match approval::request(
        "Sign this message for the generated app",
        message.as_bytes(),
        generation,
        actions,
        commands,
    )
    .await
    {
        Decision::Reject => return Ok((wallet, "Signature rejected")),
        Decision::Cancel => {
            return Err(WalletSessionError::Bridge(
                leptoskit_bridge::Error::Cancelled,
            ));
        }
        Decision::Approve => {}
    }

    show(
        local_wallet_panel(Some(&wallet.address().to_string()), true, false, commands),
        "Signing approved message",
    );
    let operation = wallet.sign_with_approval(message.as_bytes(), std::future::ready(true));

    let Some(result) = cancellable(operation, actions).await else {
        return Err(WalletSessionError::Bridge(
            leptoskit_bridge::Error::Cancelled,
        ));
    };
    let (wallet, signature) = result?;
    let signature = signature.ok_or(WalletSessionError::Malformed)?;
    verify(&wallet.address(), message.as_bytes(), &signature)
        .map_err(|_| WalletSessionError::Malformed)?;
    Ok((wallet, "Signature independently verified"))
}
