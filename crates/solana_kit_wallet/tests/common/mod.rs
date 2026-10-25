use solana_kit_wallet::*;
use std::{
    future::{Ready, ready},
    rc::Rc,
};

pub fn account() -> WalletAccount {
    WalletAccount::new(
        "display-address".into(),
        [7; 32],
        vec![SOLANA_CHAINS[3].into()],
        vec![SIGN_MESSAGE.into()],
        Some("Primary".into()),
        Some(WalletIcon::parse("data:image/png;base64,AA==").unwrap()),
    )
    .unwrap()
}

#[derive(Default)]
pub struct Backend;

impl WalletFeature for Backend {
    fn version(&self) -> &str {
        "1.0.0"
    }
}

impl Connect for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<ConnectOutput, WalletError>>;
    fn connect(&self, input: ConnectInput) -> Self::Future<'_> {
        ready(if input.silent {
            Err(rejected())
        } else {
            Ok(ConnectOutput {
                accounts: vec![account()],
            })
        })
    }
}

impl Disconnect for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<(), WalletError>>;
    fn disconnect(&self) -> Self::Future<'_> {
        ready(Ok(()))
    }
}

pub fn rejected() -> WalletError {
    WalletError {
        code: WalletErrorCode::UserRejected,
        message: "denied".into(),
        cause: None,
    }
}

impl SignTransaction for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignTransactionOutput>, WalletError>>;
    fn supported_transaction_versions(&self) -> &[TransactionVersion] {
        &[
            TransactionVersion::Legacy,
            TransactionVersion::V0,
            TransactionVersion::V1,
        ]
    }
    fn sign_transaction(&self, inputs: Vec<SignTransactionInput>) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| SignTransactionOutput {
                signed_transaction: input.transaction,
            })
            .collect()))
    }
}

impl SignAndSendTransaction for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignAndSendTransactionOutput>, WalletError>>;
    fn supported_transaction_versions(&self) -> &[TransactionVersion] {
        &[TransactionVersion::V1]
    }
    fn sign_and_send_transaction(
        &self,
        inputs: Vec<SignAndSendTransactionInput>,
    ) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| SignAndSendTransactionOutput {
                signature: input.transaction,
            })
            .collect()))
    }
}

impl SignAndSendAllTransactions for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignAndSendAllResult<WalletError>>, WalletError>>;
    fn supported_transaction_versions(&self) -> &[TransactionVersion] {
        &[TransactionVersion::V1]
    }
    fn sign_and_send_all_transactions(
        &self,
        inputs: Vec<SignAndSendTransactionInput>,
        mode: SignAndSendAllMode,
    ) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| {
                if input.transaction.is_empty() {
                    return Err(rejected());
                }

                let mut signature = input.transaction;
                signature.push(u8::from(mode == SignAndSendAllMode::Serial));
                Ok(SignAndSendTransactionOutput { signature })
            })
            .collect()))
    }
}

impl SignMessage for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignMessageOutput>, WalletError>>;
    fn sign_message(&self, inputs: Vec<SignMessageInput>) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| SignMessageOutput {
                signed_message: input.message,
                signature: vec![9],
                signature_type: Some("ed25519".into()),
            })
            .collect()))
    }
}

impl SignIn for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignInOutput>, WalletError>>;
    fn sign_in(&self, inputs: Vec<SignInInput>) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| SignInOutput {
                account: account(),
                message: SignMessageOutput {
                    signed_message: input.domain.unwrap_or_default().into_bytes(),
                    signature: vec![8],
                    signature_type: None,
                },
            })
            .collect()))
    }
}

impl SignOffchainMessage for Backend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignOffchainMessageOutput>, WalletError>>;
    fn supported_message_versions(&self) -> &[i64] {
        &[1, 2]
    }
    fn sign_offchain_message(&self, inputs: Vec<SignOffchainMessageInput>) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| SignOffchainMessageOutput {
                signed_offchain_message: input.message.into_bytes(),
                signature: vec![6],
                signature_type: None,
            })
            .collect()))
    }
}

pub struct TestWallet {
    name: String,
    icon: WalletIcon,
    features: WalletFeatures,
    accounts: Vec<WalletAccount>,
}

impl TestWallet {
    pub fn named(name: &str) -> Rc<dyn Wallet> {
        let mut features = WalletFeatures::new();
        features.insert(CONNECT, Backend).unwrap();
        Rc::new(Self {
            name: name.into(),
            icon: WalletIcon::parse("data:image/png;base64,AA==").unwrap(),
            features,
            accounts: vec![account()],
        })
    }
}

impl Wallet for TestWallet {
    fn version(&self) -> &str {
        WALLET_STANDARD_VERSION
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn icon(&self) -> &WalletIcon {
        &self.icon
    }
    fn chains(&self) -> Vec<String> {
        self.accounts[0].chains().to_vec()
    }
    fn features(&self) -> WalletFeatures {
        self.features.clone()
    }
    fn accounts(&self) -> Vec<WalletAccount> {
        self.accounts.clone()
    }
}
