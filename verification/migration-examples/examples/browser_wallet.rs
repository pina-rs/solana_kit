//! Compile-only browser wallet selection and signing example.
//!
//! The module is compiled only for `wasm32-unknown-unknown`; the native build
//! intentionally contains an empty `main` because browser wallet APIs require a
//! real `Window`.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod browser {
    use solana_kit_browser::wallet::{
        BrowserWallet, Connection, WalletController, WalletDiscovery, WalletFailure,
    };
    use solana_kit_signers::{MessageSigner, SignerConfig};
    use std::rc::Rc;

    pub async fn select_and_sign(
        discovery: Rc<WalletDiscovery>,
        selected: Rc<BrowserWallet>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let controller = WalletController::new(discovery.clone(), "solana:localnet".into())?;
        let result = connect_and_sign(&controller, selected).await;
        controller.dispose();

        result
    }

    async fn connect_and_sign(
        controller: &WalletController,
        selected: Rc<BrowserWallet>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        controller.connect(selected, false)?.await?;

        let Connection::Connected { account, .. } = controller.state().connection else {
            return Err("selected wallet has no connected account".into());
        };
        controller.select_account(account)?;

        let signer = controller.signer()?;
        let output = MessageSigner::<WalletFailure>::sign_message(
            &signer,
            b"migration example",
            SignerConfig::default(),
        )
        .await?;
        let _ = output;

        Ok(())
    }
}

fn main() {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    let _ = browser::select_and_sign;
}
