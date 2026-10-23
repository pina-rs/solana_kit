//! Canonical initialized account dispatch, distinct from named structural codecs.
use crate::{Extension, Mint, MintCodec, Multisig, MultisigCodec, PROGRAM, Token, TokenCodec};
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};

/// Initialized account family identified by canonical length and account-type byte.
#[derive(Clone, Debug, PartialEq)]
pub enum Token2022Account {
    /// A mint with only mint-affine extensions.
    Mint(Mint),
    /// A token account with only account-affine extensions.
    Token(Token),
    /// A canonical MULTISIG_LEN-byte multisignature account.
    Multisig(Multisig),
}

/// Decodes canonical initialized state owned by the standard program.
pub fn decode_account(
    owner: Address,
    data: &[u8],
    limits: Limits,
) -> Result<Token2022Account, Error> {
    decode_account_with_program(PROGRAM, owner, data, limits)
}

/// Decodes state using an explicit expected owner; no owner is inferred from bytes.
pub fn decode_account_with_program(
    program: Address,
    owner: Address,
    data: &[u8],
    limits: Limits,
) -> Result<Token2022Account, Error> {
    if owner != program {
        return Err(Error::InvalidValue);
    }

    let account = match data.len() {
        MINT_BASE_LEN => Token2022Account::Mint(MintCodec.decode(data, limits)?),
        TOKEN_BASE_LEN => Token2022Account::Token(TokenCodec.decode(data, limits)?),
        MULTISIG_LEN => Token2022Account::Multisig(MultisigCodec.decode(data, limits)?),

        EXTENDED_HEADER_LEN.. => match data[TOKEN_BASE_LEN] {
            1 => Token2022Account::Mint(MintCodec.decode(data, limits)?),
            2 => Token2022Account::Token(TokenCodec.decode(data, limits)?),
            _ => return Err(Error::InvalidDiscriminator),
        },
        _ => return Err(Error::InvalidValue),
    };

    account.validate()?;
    Ok(account)
}

impl Token2022Account {
    /// Checks initialization, signer bounds, extension uniqueness and base affinity.
    pub fn validate(&self) -> Result<(), Error> {
        match self {
            Self::Mint(value) if value.is_initialized => {
                validate_extensions(value.extensions.as_deref(), true)
            }

            Self::Token(value) if value.state != crate::AccountState::Uninitialized => {
                validate_extensions(value.extensions.as_deref(), false)
            }

            Self::Multisig(value)
                if value.is_initialized
                    && value.m > 0
                    && value.m <= value.n
                    && value.n <= MAX_SIGNERS =>
            {
                Ok(())
            }
            _ => Err(Error::InvalidValue),
        }
    }

    /// Encodes validated canonical state, adding source-required MULTISIG_LEN-byte padding.
    pub fn encode(&self, max_bytes: usize) -> Result<Vec<u8>, Error> {
        self.validate()?;

        let mut bytes = match self {
            Self::Mint(value) => MintCodec.encode(value, max_bytes)?,
            Self::Token(value) => TokenCodec.encode(value, max_bytes)?,
            Self::Multisig(value) => return MultisigCodec.encode(value, max_bytes),
        };

        // The pinned protocol reserves exactly355 bytes for Multisig. Extended
        // Mint/Token allocations of that length add a u16 terminator instead.
        if bytes.len() == MULTISIG_LEN {
            if max_bytes < PADDED_MULTISIG_LEN {
                return Err(Error::LimitExceeded);
            }

            bytes.extend_from_slice(&[0, 0]);
        }

        Ok(bytes)
    }
}

fn validate_extensions(extensions: Option<&[Extension]>, mint: bool) -> Result<(), Error> {
    let mut seen = 0u32;

    for extension in extensions.unwrap_or_default() {
        let (tag, belongs_to_mint) = crate::generated::extension_identity(extension)?;
        let bit = 1u32
            .checked_shl(u32::from(tag))
            .ok_or(Error::InvalidValue)?;

        if belongs_to_mint != mint || seen & bit != 0 {
            return Err(Error::InvalidValue);
        }

        seen |= bit;
    }

    if mint {
        validate_mint_combination(seen)?;
    }

    Ok(())
}

fn validate_mint_combination(tags: u32) -> Result<(), Error> {
    let has = |tag: u32| -> bool { (tags & (1u32 << tag)) != 0u32 };
    let fee = has(TRANSFER_FEE_CONFIG_TAG);
    let confidential = has(CONFIDENTIAL_MINT_TAG);
    let confidential_fee = has(CONFIDENTIAL_FEE_TAG);
    let mint_burn = has(CONFIDENTIAL_MINT_BURN_TAG);

    if confidential_fee != (fee && confidential)
        || mint_burn && !confidential
        || has(SCALED_UI_TAG) && has(INTEREST_BEARING_TAG)
        || has(NON_TRANSFERABLE_TAG) && confidential && !mint_burn
    {
        return Err(Error::InvalidValue);
    }

    Ok(())
}

/// Computes canonical mint allocation including TLV framing and reserved-size padding.
pub fn mint_size(extensions: Option<&[Extension]>) -> Result<usize, Error> {
    account_size(extensions, true)
}

/// Computes canonical token allocation including TLV framing and reserved-size padding.
pub fn token_size(extensions: Option<&[Extension]>) -> Result<usize, Error> {
    account_size(extensions, false)
}

/// Account type and hidden padding change only when an extension tail is present.
fn account_size(extensions: Option<&[Extension]>, mint: bool) -> Result<usize, Error> {
    validate_extensions(extensions, mint)?;
    let Some(extensions) = extensions else {
        return Ok(if mint { MINT_BASE_LEN } else { TOKEN_BASE_LEN });
    };
    let mut size = 166usize;

    for extension in extensions {
        size = size
            .checked_add(crate::ExtensionCodec.encoded_len(extension)?)
            .ok_or(Error::Overflow)?;
    }

    Ok(if size == MULTISIG_LEN {
        PADDED_MULTISIG_LEN
    } else {
        size
    })
}

const MINT_BASE_LEN: usize = 82;
const TOKEN_BASE_LEN: usize = 165;
const EXTENDED_HEADER_LEN: usize = 166;
const MULTISIG_LEN: usize = 355;
const PADDED_MULTISIG_LEN: usize = 357;
const MAX_SIGNERS: u8 = 11;
const TRANSFER_FEE_CONFIG_TAG: u32 = 1;
const CONFIDENTIAL_MINT_TAG: u32 = 4;

const CONFIDENTIAL_FEE_TAG: u32 = 16;
const CONFIDENTIAL_MINT_BURN_TAG: u32 = 24;
const SCALED_UI_TAG: u32 = 25;
const INTEREST_BEARING_TAG: u32 = 10;
const NON_TRANSFERABLE_TAG: u32 = 9;
