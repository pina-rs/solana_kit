//! Immutable-value account mutations; numeric invariants are encoded by unsigned types.

use serde::{Serialize, Serializer};
use serde_json::Value;
use solana_kit_core::Address;

use crate::{hex::HexBytes, types::*};

/// Optional fields for one arbitrary account mutation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    /// Lamport balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lamports: Option<u64>,
    /// Complete account bytes encoded in lowercase hexadecimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HexBytes>,
    /// Owner program, serialized as a canonical address.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "owner")]
    pub owner: Option<Address>,
    /// Rent epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rent_epoch: Option<u64>,
    /// Executable flag; false is retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<bool>,
}

fn owner<S: Serializer>(address: &Option<Address>, serializer: S) -> Result<S::Ok, S::Error> {
    address
        .as_ref()
        .map(ToString::to_string)
        .serialize(serializer)
}

/// `surfnet_setAccount` request, with the same positional object as Dart.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use]
pub struct SetAccount {
    /// Account to mutate.
    pub address: Address,
    /// Mutation fields.
    pub update: AccountUpdate,
}

impl SetAccount {
    /// Starts an empty account mutation.
    pub fn new(address: Address) -> Self {
        Self {
            address,
            update: AccountUpdate::default(),
        }
    }

    /// Sets a precise lamport balance.
    pub fn with_lamports(mut self, lamports: u64) -> Self {
        self.update.lamports = Some(lamports);
        self
    }

    /// Takes ownership of the complete account data.
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.update.data = Some(HexBytes(data));
        self
    }

    /// Sets the account's owner program.
    pub fn with_owner(mut self, owner: Address) -> Self {
        self.update.owner = Some(owner);
        self
    }

    /// Sets the account's rent epoch.
    pub fn with_rent_epoch(mut self, epoch: u64) -> Self {
        self.update.rent_epoch = Some(epoch);
        self
    }

    /// Sets or clears the executable flag.
    pub fn with_executable(mut self, executable: bool) -> Self {
        self.update.executable = Some(executable);
        self
    }
}

impl Serialize for SetAccount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.address.to_string(), &self.update).serialize(serializer)
    }
}

method!(SetAccount, "surfnet_setAccount", Value);

/// `surfnet_setTokenAccount` request with an optional final token program.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use]
pub struct SetTokenAccount {
    /// Token owner wallet.
    pub owner: Address,
    /// Token mint.
    pub mint: Address,
    /// Advanced token mutation fields.
    pub update: SetTokenAccountUpdate,
    /// Explicit token program; omission preserves the builder's source behavior.
    pub token_program: Option<Address>,
}

impl SetTokenAccount {
    /// Starts a token account mutation without an explicit program.
    pub fn new(owner: Address, mint: Address) -> Self {
        Self {
            owner,
            mint,
            update: SetTokenAccountUpdate::default(),
            token_program: None,
        }
    }

    /// Sets the token amount.
    pub fn with_amount(mut self, amount: u64) -> Self {
        self.update.amount = Some(amount);
        self
    }

    /// Replaces the delegate.
    pub fn with_delegate(mut self, delegate: Address) -> Self {
        self.update.delegate = AuthorityUpdate::Set(delegate);
        self
    }

    /// Clears the delegate using the provider's literal string convention.
    pub fn without_delegate(mut self) -> Self {
        self.update.delegate = AuthorityUpdate::Clear;
        self
    }

    /// Sets the account state string without inventing a closed provider enum.
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.update.state = Some(state.into());
        self
    }

    /// Sets the delegated amount.
    pub fn with_delegated_amount(mut self, amount: u64) -> Self {
        self.update.delegated_amount = Some(amount);
        self
    }

    /// Replaces the close authority.
    pub fn with_close_authority(mut self, authority: Address) -> Self {
        self.update.close_authority = AuthorityUpdate::Set(authority);
        self
    }

    /// Clears the close authority using the literal string convention.
    pub fn without_close_authority(mut self) -> Self {
        self.update.close_authority = AuthorityUpdate::Clear;
        self
    }

    /// Sets the confidential-transfer extension fields.
    pub fn with_confidential_transfer(mut self, update: ConfidentialTransferAccountUpdate) -> Self {
        self.update.confidential = Some(update);
        self
    }

    /// Chooses the token program explicitly.
    pub fn with_token_program(mut self, program: Address) -> Self {
        self.token_program = Some(program);
        self
    }
}

impl Serialize for SetTokenAccount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence =
            serializer.serialize_seq(Some(3 + usize::from(self.token_program.is_some())))?;
        sequence.serialize_element(&self.owner.to_string())?;
        sequence.serialize_element(&self.mint.to_string())?;
        sequence.serialize_element(&self.update)?;

        if let Some(program) = self.token_program {
            sequence.serialize_element(&program.to_string())?;
        }

        sequence.end()
    }
}

method!(SetTokenAccount, "surfnet_setTokenAccount", Value);

macro_rules! account_options {
    ($name:ident, $method:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[must_use]
        pub struct $name {
            /// Account to reset or stream.
            pub address: Address,
            /// Missing omits the options argument entirely.
            pub include_owned_accounts: Option<bool>,
        }

        impl $name {
            /// Starts a request without an options argument.
            pub const fn new(address: Address) -> Self {
                Self {
                    address,
                    include_owned_accounts: None,
                }
            }

            /// Includes an explicit true or false owned-account flag.
            pub const fn with_include_owned_accounts(mut self, include: bool) -> Self {
                self.include_owned_accounts = Some(include);
                self
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let options = self
                    .include_owned_accounts
                    .map(|include| ResetAccountOptions {
                        include_owned_accounts: Some(include),
                    });
                crate::wire::trailing(self.address.to_string(), options.as_ref(), serializer)
            }
        }

        method!($name, $method, Value);
    };
}

account_options!(
    ResetAccount,
    "surfnet_resetAccount",
    "Reset an account to upstream state."
);
account_options!(
    StreamAccount,
    "surfnet_streamAccount",
    "Stream an account from its upstream."
);
