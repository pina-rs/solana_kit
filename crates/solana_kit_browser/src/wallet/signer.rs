use super::{
    BrowserWallet, WalletFailure,
    controller::{Connection, Inner, OperationGuard},
    interop::*,
    operation::Current,
};
use js_sys::{Array, Object, Uint8Array};
use solana_kit_core::{
    Address, Signature,
    transaction::{Transaction, Version},
};
use solana_kit_signers::{
    MessageSigner, SignatureOutput, Signer, TransactionModifyingSigner, TransactionPartialSigner,
    TransactionSendingSigner,
};
use solana_kit_wallet::{
    SIGN_AND_SEND_TRANSACTION, SIGN_MESSAGE, SIGN_TRANSACTION, Wallet, WalletAccount,
    WalletErrorCode,
};
use std::{
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
};
use wasm_bindgen::JsValue;

type Signing<'a, T> = Pin<Box<dyn Future<Output = Result<T, WalletFailure>> + 'a>>;

/// A typed Kit capability bound to one live controller authorization.
///
/// `WalletFailure` retains local JavaScript causes; default native signer errors
/// stay Send. The low-level Kit traits expose this failure type explicitly.
/// Replacing the account, wallet, capabilities or controller invalidates this
/// signer. Wallet output is cryptographically verified before it is returned.
pub struct WalletSigner {
    inner: Weak<Inner>,
    authorization: Rc<()>,
    wallet: Rc<BrowserWallet>,
    account: WalletAccount,
    address: Address,
    chain: String,
}

impl WalletSigner {
    pub(super) fn new(inner: &Rc<Inner>) -> Result<Self, WalletFailure> {
        let Connection::Connected { wallet, account } = inner.state.borrow().connection.clone()
        else {
            return Err(WalletFailure::disconnected());
        };
        let address: Address = account
            .address()
            .parse()
            .map_err(|error| WalletFailure::caused("invalid account address", error))?;

        if address != Address::from_bytes(*account.public_key()) {
            return Err(WalletFailure::invalid());
        }

        Ok(Self {
            inner: Rc::downgrade(inner),
            authorization: inner.authorization.borrow().clone(),
            wallet,
            account,
            address,
            chain: inner.chain.clone(),
        })
    }

    fn begin(&self, identifier: &str) -> Result<OperationGuard, WalletFailure> {
        let inner = self
            .inner
            .upgrade()
            .ok_or_else(WalletFailure::disconnected)?;
        let current = inner.state.borrow().connection.clone();
        let Connection::Connected { wallet, account } = current else {
            return Err(WalletFailure::disconnected());
        };

        if !Rc::ptr_eq(&self.authorization, &inner.authorization.borrow())
            || !wallet.same(&self.wallet)
            || account != self.account
        {
            return Err(WalletFailure::disconnected());
        }

        if !self
            .account
            .features()
            .iter()
            .any(|feature| feature == identifier)
            || !self.wallet.chains().contains(&self.chain)
        {
            return Err(WalletFailure::new(
                WalletErrorCode::UnsupportedFeature,
                "account does not authorize this wallet capability",
            ));
        }

        self.wallet.raw_account(&self.account)?;
        self.wallet.feature(identifier)?;
        inner.begin()
    }

    fn request(&self) -> Result<Object, WalletFailure> {
        let request = Object::new();
        set(
            &request,
            "account",
            &self.wallet.raw_account(&self.account)?,
        )?;
        Ok(request)
    }

    async fn invoke(
        &self,
        identifier: &str,
        method: &str,
        request: Object,
        guard: &OperationGuard,
    ) -> Result<JsValue, WalletFailure> {
        let feature = self.wallet.feature(identifier)?;
        let args = Array::of1(&request);
        one(Current::new(guard.token.clone(), promise(&feature.raw, method, &args)).await?)
    }

    fn signature(&self, bytes: &[u8], signature: Vec<u8>) -> Result<Signature, WalletFailure> {
        let signature =
            Signature::from_bytes(signature.try_into().map_err(|_| WalletFailure::invalid())?);
        solana_kit_core::crypto::verify(&self.address, bytes, &signature)
            .map_err(|error| WalletFailure::caused("wallet signature did not verify", error))?;
        Ok(signature)
    }

    async fn message(&self, message: &[u8]) -> Result<SignatureOutput, WalletFailure> {
        let mut guard = self.begin(SIGN_MESSAGE)?;
        let request = self.request()?;
        set(&request, "message", &Uint8Array::from(message))?;
        let result = self
            .invoke(SIGN_MESSAGE, "signMessage", request, &guard)
            .await
            .and_then(|output| self.message_output(&output, message));
        guard.finish(result)
    }

    fn message_output(
        &self,
        output: &JsValue,
        expected: &[u8],
    ) -> Result<SignatureOutput, WalletFailure> {
        let message = bytes(output, "signedMessage")?;
        let algorithm = optional_string(output, "signatureType")?;

        if message != expected || algorithm.is_some_and(|value| value != "ed25519") {
            return Err(WalletFailure::invalid());
        }

        let signature = self.signature(&message, bytes(output, "signature")?)?;
        Ok(SignatureOutput::new(self.address, &message, signature))
    }

    fn transaction_request(
        &self,
        identifier: &str,
        transaction: &Transaction,
    ) -> Result<Object, WalletFailure> {
        let feature = self.wallet.feature(identifier)?;
        let versions = array(get(&feature.raw, "supportedTransactionVersions")?)?;

        let version = match transaction.message().version() {
            Version::Legacy => JsValue::from_str("legacy"),
            Version::V0 => JsValue::from_f64(0.0),
            Version::V1 => JsValue::from_f64(1.0),
        };

        if !versions
            .iter()
            .any(|advertised| Object::is(&advertised, &version))
        {
            return Err(WalletFailure::new(
                WalletErrorCode::UnsupportedFeature,
                "wallet does not support this transaction version",
            ));
        }

        let encoded = transaction
            .encode(false)
            .map_err(|error| WalletFailure::caused("invalid input transaction", error))?;
        let request = self.request()?;
        set(
            &request,
            "transaction",
            &Uint8Array::from(encoded.as_slice()),
        )?;
        set(&request, "chain", &JsValue::from_str(&self.chain))?;
        Ok(request)
    }

    async fn transaction(
        &self,
        transaction: &Transaction,
        modifying: bool,
    ) -> Result<Transaction, WalletFailure> {
        let mut guard = self.begin(SIGN_TRANSACTION)?;
        let request = self.transaction_request(SIGN_TRANSACTION, transaction)?;
        let result = self
            .invoke(SIGN_TRANSACTION, "signTransaction", request, &guard)
            .await
            .and_then(|output| self.transaction_output(&output))
            .and_then(|signed| {
                if !modifying && signed.message().bytes() != transaction.message().bytes() {
                    return Err(WalletFailure::invalid());
                }

                Ok(signed)
            });
        guard.finish(result)
    }

    fn transaction_output(&self, output: &JsValue) -> Result<Transaction, WalletFailure> {
        let transaction = Transaction::decode(&bytes(output, "signedTransaction")?)
            .map_err(|error| WalletFailure::caused("invalid signed transaction", error))?;
        self.transaction_signature(&transaction)?;
        self.validate_transaction_signatures(&transaction)?;
        Ok(transaction)
    }

    fn validate_transaction_signatures(
        &self,
        transaction: &Transaction,
    ) -> Result<(), WalletFailure> {
        for (address, slot) in transaction
            .message()
            .signers()
            .iter()
            .zip(transaction.signatures())
        {
            self.validate_transaction_slot(transaction, *address, *slot)?;
        }

        Ok(())
    }

    fn validate_transaction_slot(
        &self,
        transaction: &Transaction,
        address: Address,
        slot: Option<Signature>,
    ) -> Result<(), WalletFailure> {
        match slot {
            Some(signature) => {
                solana_kit_core::crypto::verify(&address, transaction.message().bytes(), &signature)
                    .map_err(|error| {
                        WalletFailure::caused("invalid wallet transaction signature", error)
                    })
            }
            None if address == self.address => Err(WalletFailure::invalid()),
            None => Ok(()),
        }
    }

    fn transaction_signature(&self, transaction: &Transaction) -> Result<Signature, WalletFailure> {
        transaction
            .message()
            .signers()
            .iter()
            .zip(transaction.signatures())
            .find(|(address, _)| **address == self.address)
            .and_then(|(_, signature)| *signature)
            .ok_or_else(WalletFailure::invalid)
    }

    async fn partial(&self, transaction: &Transaction) -> Result<SignatureOutput, WalletFailure> {
        let signed = self.transaction(transaction, false).await?;

        if signed.message().bytes() != transaction.message().bytes() {
            return Err(WalletFailure::invalid());
        }

        Ok(SignatureOutput::new(
            self.address,
            signed.message().bytes(),
            self.transaction_signature(&signed)?,
        ))
    }

    async fn send(&self, transaction: &Transaction) -> Result<Signature, WalletFailure> {
        let mut guard = self.begin(SIGN_AND_SEND_TRANSACTION)?;
        let request = self.transaction_request(SIGN_AND_SEND_TRANSACTION, transaction)?;
        let result = self
            .invoke(
                SIGN_AND_SEND_TRANSACTION,
                "signAndSendTransaction",
                request,
                &guard,
            )
            .await
            .and_then(|output| self.submission_output(&output, transaction));
        guard.finish(result)
    }

    fn submission_output(
        &self,
        output: &JsValue,
        transaction: &Transaction,
    ) -> Result<Signature, WalletFailure> {
        let signature = Signature::from_bytes(
            bytes(output, "signature")?
                .try_into()
                .map_err(|_| WalletFailure::invalid())?,
        );
        let payer = transaction
            .message()
            .signers()
            .first()
            .ok_or_else(WalletFailure::invalid)?;
        let verified =
            solana_kit_core::crypto::verify(payer, transaction.message().bytes(), &signature)
                .is_ok();
        let existing = transaction.signatures().first().and_then(Option::as_ref);

        if !verified || existing.is_some_and(|expected| *expected != signature) {
            return Err(WalletFailure::invalid());
        }

        Ok(signature)
    }
}

impl Signer for WalletSigner {
    fn address(&self) -> Address {
        self.address
    }
}

impl MessageSigner<WalletFailure> for WalletSigner {
    type Sign<'a> = Signing<'a, SignatureOutput>;
    fn sign_message<'a>(&'a self, bytes: &'a [u8]) -> Self::Sign<'a> {
        Box::pin(self.message(bytes))
    }
}

impl TransactionPartialSigner<WalletFailure> for WalletSigner {
    type Sign<'a> = Signing<'a, SignatureOutput>;
    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a> {
        Box::pin(self.partial(transaction))
    }
}

impl TransactionModifyingSigner<WalletFailure> for WalletSigner {
    type Modify<'a> = Signing<'a, Transaction>;
    fn modify_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Modify<'a> {
        Box::pin(self.transaction(transaction, true))
    }
}

impl TransactionSendingSigner<WalletFailure> for WalletSigner {
    type Send<'a> = Signing<'a, Signature>;
    fn send_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Send<'a> {
        Box::pin(self.send(transaction))
    }
}
