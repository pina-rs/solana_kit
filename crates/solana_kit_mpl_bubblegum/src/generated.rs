// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    152, 139, 128, 235, 121, 53, 40, 105, 178, 36, 116, 95, 89, 221, 191, 138, 38, 88, 202, 19,
    220, 104, 129, 33, 38, 53, 28, 174, 7, 193, 165, 165,
]);
const PROGRAM_ERROR_6000: u32 = 6000;
const PROGRAM_ERROR_6001: u32 = 6001;
const PROGRAM_ERROR_6002: u32 = 6002;
const PROGRAM_ERROR_6003: u32 = 6003;
const PROGRAM_ERROR_6004: u32 = 6004;
const PROGRAM_ERROR_6005: u32 = 6005;
const PROGRAM_ERROR_6006: u32 = 6006;
const PROGRAM_ERROR_6007: u32 = 6007;
const PROGRAM_ERROR_6008: u32 = 6008;
const PROGRAM_ERROR_6009: u32 = 6009;
const PROGRAM_ERROR_6010: u32 = 6010;
const PROGRAM_ERROR_6011: u32 = 6011;
const PROGRAM_ERROR_6012: u32 = 6012;
const PROGRAM_ERROR_6013: u32 = 6013;
const PROGRAM_ERROR_6014: u32 = 6014;
const PROGRAM_ERROR_6015: u32 = 6015;
const PROGRAM_ERROR_6016: u32 = 6016;
const PROGRAM_ERROR_6017: u32 = 6017;
const PROGRAM_ERROR_6018: u32 = 6018;
const PROGRAM_ERROR_6019: u32 = 6019;
const PROGRAM_ERROR_6020: u32 = 6020;
const PROGRAM_ERROR_6021: u32 = 6021;
const PROGRAM_ERROR_6022: u32 = 6022;
const PROGRAM_ERROR_6023: u32 = 6023;
const PROGRAM_ERROR_6024: u32 = 6024;
const PROGRAM_ERROR_6025: u32 = 6025;
const PROGRAM_ERROR_6026: u32 = 6026;
const PROGRAM_ERROR_6027: u32 = 6027;
const PROGRAM_ERROR_6028: u32 = 6028;
const PROGRAM_ERROR_6029: u32 = 6029;
const PROGRAM_ERROR_6030: u32 = 6030;
const PROGRAM_ERROR_6031: u32 = 6031;
const PROGRAM_ERROR_6032: u32 = 6032;
const PROGRAM_ERROR_6033: u32 = 6033;
const PROGRAM_ERROR_6034: u32 = 6034;
const PROGRAM_ERROR_6035: u32 = 6035;
const PROGRAM_ERROR_6036: u32 = 6036;
const PROGRAM_ERROR_6037: u32 = 6037;
const PROGRAM_ERROR_6038: u32 = 6038;
const PROGRAM_ERROR_6039: u32 = 6039;
const PROGRAM_ERROR_6040: u32 = 6040;
const PROGRAM_ERROR_6041: u32 = 6041;
const PROGRAM_ERROR_6042: u32 = 6042;
const PROGRAM_ERROR_6043: u32 = 6043;
const PROGRAM_ERROR_6044: u32 = 6044;
const PROGRAM_ERROR_6045: u32 = 6045;
const PROGRAM_ERROR_6046: u32 = 6046;
const PROGRAM_ERROR_6047: u32 = 6047;
const PROGRAM_ERROR_6048: u32 = 6048;
const PROGRAM_ERROR_6049: u32 = 6049;
const PROGRAM_ERROR_6050: u32 = 6050;
const PROGRAM_ERROR_6051: u32 = 6051;
const PROGRAM_ERROR_6052: u32 = 6052;
const PROGRAM_ERROR_6053: u32 = 6053;
const PROGRAM_ERROR_6054: u32 = 6054;
const PROGRAM_ERROR_6055: u32 = 6055;
const PROGRAM_ERROR_6056: u32 = 6056;
const PROGRAM_ERROR_6057: u32 = 6057;
const PROGRAM_ERROR_6058: u32 = 6058;
const PROGRAM_ERROR_6059: u32 = 6059;

struct CheckedCounted<P, C> {
    prefix: P,
    item: C,
    minimum: usize,
}
impl<P: Encoder<u32>, C: Encoder<T>, T> Encoder<[T]> for CheckedCounted<P, C> {
    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        Counted {
            prefix: &self.prefix,
            item: &self.item,
        }
        .encoded_len(value)
    }
    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        Counted {
            prefix: &self.prefix,
            item: &self.item,
        }
        .write(value, writer)
    }
}
impl<P: Decoder<u32>, C: Decoder<T>, T> Decoder<Vec<T>> for CheckedCounted<P, C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        let count = usize::try_from(self.prefix.read(reader)?).map_err(|_| Error::Overflow)?;
        let minimum = count.checked_mul(self.minimum).ok_or(Error::Overflow)?;

        if minimum > reader.remaining().len() {
            return Err(Error::Truncated);
        }

        solana_kit_core::codecs::Sequence {
            item: &self.item,
            count: solana_kit_core::codecs::Count::Fixed(count),
        }
        .read(reader)
    }
}
fn role_matches(
    program: Address,
    meta: &AccountMeta,
    expected: &(bool, Option<bool>, bool),
) -> bool {
    let (writable, signer, optional) = *expected;

    if optional && meta.address == program {
        return meta.role == AccountRole::Readonly;
    }

    meta.role.is_writable() == writable
        && signer.is_none_or(|signer| meta.role.is_signer() == signer)
}
/// Validated data for `Creator`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Creator {
    /// The `address` wire field.
    pub address: Address,
    /// The `verified` wire field.
    pub verified: bool,
    /// The `share` wire field.
    pub share: u8,
}
/// Bounded codec for [`Creator`].
#[derive(Clone, Copy, Debug)]
pub struct CreatorCodec;
impl Encoder<Creator> for CreatorCodec {
    fn encoded_len(&self, value: &Creator) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.verified)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.share)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Creator, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Bool.write(&value.verified, writer)?;
        Number::<u8>::LE.write(&value.share, writer)?;
        Ok(())
    }
}
impl Decoder<Creator> for CreatorCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Creator, Error> {
        Ok(Creator {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            verified: Bool.read(reader)?,
            share: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `Collection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Collection {
    /// The `verified` wire field.
    pub verified: bool,
    /// The `key` wire field.
    pub key: Address,
}
/// Bounded codec for [`Collection`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionCodec;
impl Encoder<Collection> for CollectionCodec {
    fn encoded_len(&self, value: &Collection) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Bool.encoded_len(&value.verified)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.key)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Collection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Bool.write(&value.verified, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.key, writer)?;
        Ok(())
    }
}
impl Decoder<Collection> for CollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Collection, Error> {
        Ok(Collection {
            verified: Bool.read(reader)?,
            key: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
/// Validated data for `Uses`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uses {
    /// The `use_method` wire field.
    pub use_method: UseMethod,
    /// The `remaining` wire field.
    pub remaining: u64,
    /// The `total` wire field.
    pub total: u64,
}
/// Bounded codec for [`Uses`].
#[derive(Clone, Copy, Debug)]
pub struct UsesCodec;
impl Encoder<Uses> for UsesCodec {
    fn encoded_len(&self, value: &Uses) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(UseMethodCodec.encoded_len(&value.use_method)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.remaining)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.total)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Uses, writer: &mut Writer<'_>) -> Result<(), Error> {
        UseMethodCodec.write(&value.use_method, writer)?;
        Number::<u64>::LE.write(&value.remaining, writer)?;
        Number::<u64>::LE.write(&value.total, writer)?;
        Ok(())
    }
}
impl Decoder<Uses> for UsesCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Uses, Error> {
        Ok(Uses {
            use_method: UseMethodCodec.read(reader)?,
            remaining: Number::<u64>::LE.read(reader)?,
            total: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `MetadataArgs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataArgs {
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seller_fee_basis_points` wire field.
    pub seller_fee_basis_points: u16,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: bool,
    /// The `is_mutable` wire field.
    pub is_mutable: bool,
    /// The `edition_nonce` wire field.
    pub edition_nonce: Option<u8>,
    /// The `token_standard` wire field.
    pub token_standard: Option<TokenStandard>,
    /// The `collection` wire field.
    pub collection: Option<Collection>,
    /// The `uses` wire field.
    pub uses: Option<Uses>,
    /// The `token_program_version` wire field.
    pub token_program_version: TokenProgramVersion,
    /// The `creators` wire field.
    pub creators: Vec<Creator>,
}
/// Bounded codec for [`MetadataArgs`].
#[derive(Clone, Copy, Debug)]
pub struct MetadataArgsCodec;
impl Encoder<MetadataArgs> for MetadataArgsCodec {
    fn encoded_len(&self, value: &MetadataArgs) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.name.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.symbol.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.seller_fee_basis_points)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.edition_nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(TokenStandardCodec).encoded_len(&value.token_standard)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(CollectionCodec).encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(UsesCodec).encoded_len(&value.uses)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(TokenProgramVersionCodec.encoded_len(&value.token_program_version)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: CreatorCodec,
                    minimum: 34,
                }
                .encoded_len(value.creators.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MetadataArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.name.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.symbol.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.uri.as_str(), writer)?;
        Number::<u16>::LE.write(&value.seller_fee_basis_points, writer)?;
        Bool.write(&value.primary_sale_happened, writer)?;
        Bool.write(&value.is_mutable, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.edition_nonce, writer)?;
        OptionCodec::u8(TokenStandardCodec).write(&value.token_standard, writer)?;
        OptionCodec::u8(CollectionCodec).write(&value.collection, writer)?;
        OptionCodec::u8(UsesCodec).write(&value.uses, writer)?;
        TokenProgramVersionCodec.write(&value.token_program_version, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 34,
        }
        .write(value.creators.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<MetadataArgs> for MetadataArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MetadataArgs, Error> {
        Ok(MetadataArgs {
            name: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            symbol: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            uri: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            seller_fee_basis_points: Number::<u16>::LE.read(reader)?,
            primary_sale_happened: Bool.read(reader)?,
            is_mutable: Bool.read(reader)?,
            edition_nonce: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            token_standard: OptionCodec::u8(TokenStandardCodec).read(reader)?,
            collection: OptionCodec::u8(CollectionCodec).read(reader)?,
            uses: OptionCodec::u8(UsesCodec).read(reader)?,
            token_program_version: TokenProgramVersionCodec.read(reader)?,
            creators: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 34,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `MetadataArgsV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataArgsV2 {
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seller_fee_basis_points` wire field.
    pub seller_fee_basis_points: u16,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: bool,
    /// The `is_mutable` wire field.
    pub is_mutable: bool,
    /// The `token_standard` wire field.
    pub token_standard: Option<TokenStandard>,
    /// The `creators` wire field.
    pub creators: Vec<Creator>,
    /// The `collection` wire field.
    pub collection: Option<Address>,
}
/// Bounded codec for [`MetadataArgsV2`].
#[derive(Clone, Copy, Debug)]
pub struct MetadataArgsV2Codec;
impl Encoder<MetadataArgsV2> for MetadataArgsV2Codec {
    fn encoded_len(&self, value: &MetadataArgsV2) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.name.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.symbol.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }
                .encoded_len(value.uri.as_str())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.seller_fee_basis_points)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(TokenStandardCodec).encoded_len(&value.token_standard)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: CreatorCodec,
                    minimum: 34,
                }
                .encoded_len(value.creators.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.collection)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MetadataArgsV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.name.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.symbol.as_str(), writer)?;
        ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }
        .write(value.uri.as_str(), writer)?;
        Number::<u16>::LE.write(&value.seller_fee_basis_points, writer)?;
        Bool.write(&value.primary_sale_happened, writer)?;
        Bool.write(&value.is_mutable, writer)?;
        OptionCodec::u8(TokenStandardCodec).write(&value.token_standard, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 34,
        }
        .write(value.creators.as_slice(), writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.collection, writer)?;
        Ok(())
    }
}
impl Decoder<MetadataArgsV2> for MetadataArgsV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MetadataArgsV2, Error> {
        Ok(MetadataArgsV2 {
            name: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            symbol: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            uri: ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }
            .read(reader)?,
            seller_fee_basis_points: Number::<u16>::LE.read(reader)?,
            primary_sale_happened: Bool.read(reader)?,
            is_mutable: Bool.read(reader)?,
            token_standard: OptionCodec::u8(TokenStandardCodec).read(reader)?,
            creators: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 34,
            }
            .read(reader)?,
            collection: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgs {
    /// The `name` wire field.
    pub name: Option<String>,
    /// The `symbol` wire field.
    pub symbol: Option<String>,
    /// The `uri` wire field.
    pub uri: Option<String>,
    /// The `creators` wire field.
    pub creators: Option<Vec<Creator>>,
    /// The `seller_fee_basis_points` wire field.
    pub seller_fee_basis_points: Option<u16>,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: Option<bool>,
    /// The `is_mutable` wire field.
    pub is_mutable: Option<bool>,
}
/// Bounded codec for [`UpdateArgs`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsCodec;
impl Encoder<UpdateArgs> for UpdateArgsCodec {
    fn encoded_len(&self, value: &UpdateArgs) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.name)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.symbol)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }))
                .encoded_len(&value.uri)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: CreatorCodec,
                    minimum: 34,
                }))
                .encoded_len(&value.creators)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Number::<u16>::LE).encoded_len(&value.seller_fee_basis_points)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.name, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.symbol, writer)?;
        OptionCodec::u8(OwnedString(ByteSizePrefix {
            item: Utf8,
            prefix: Mapped::new(
                Number::<u32>::LE,
                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                |value: u32| Ok::<_, Error>(u64::from(value)),
            ),
            consumption: FrameConsumption::Exact,
        }))
        .write(&value.uri, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 34,
        }))
        .write(&value.creators, writer)?;
        OptionCodec::u8(Number::<u16>::LE).write(&value.seller_fee_basis_points, writer)?;
        OptionCodec::u8(Bool).write(&value.primary_sale_happened, writer)?;
        OptionCodec::u8(Bool).write(&value.is_mutable, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgs> for UpdateArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateArgs, Error> {
        Ok(UpdateArgs {
            name: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            symbol: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            uri: OptionCodec::u8(OwnedString(ByteSizePrefix {
                item: Utf8,
                prefix: Mapped::new(
                    Number::<u32>::LE,
                    |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u32| Ok::<_, Error>(u64::from(value)),
                ),
                consumption: FrameConsumption::Exact,
            }))
            .read(reader)?,
            creators: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 34,
            }))
            .read(reader)?,
            seller_fee_basis_points: OptionCodec::u8(Number::<u16>::LE).read(reader)?,
            primary_sale_happened: OptionCodec::u8(Bool).read(reader)?,
            is_mutable: OptionCodec::u8(Bool).read(reader)?,
        })
    }
}
/// Validated data for `LeafSchemaEvent`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeafSchemaEvent {
    /// The `event_type` wire field.
    pub event_type: BubblegumEventType,
    /// The `version` wire field.
    pub version: Version,
    /// The `schema` wire field.
    pub schema: LeafSchema,
    /// The `leaf_hash` wire field.
    pub leaf_hash: [u8; 32],
}
/// Bounded codec for [`LeafSchemaEvent`].
#[derive(Clone, Copy, Debug)]
pub struct LeafSchemaEventCodec;
impl Encoder<LeafSchemaEvent> for LeafSchemaEventCodec {
    fn encoded_len(&self, value: &LeafSchemaEvent) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(BubblegumEventTypeCodec.encoded_len(&value.event_type)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(VersionCodec.encoded_len(&value.version)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(LeafSchemaCodec.encoded_len(&value.schema)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.leaf_hash)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LeafSchemaEvent, writer: &mut Writer<'_>) -> Result<(), Error> {
        BubblegumEventTypeCodec.write(&value.event_type, writer)?;
        VersionCodec.write(&value.version, writer)?;
        LeafSchemaCodec.write(&value.schema, writer)?;
        FixedBytes::<32>.write(&value.leaf_hash, writer)?;
        Ok(())
    }
}
impl Decoder<LeafSchemaEvent> for LeafSchemaEventCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LeafSchemaEvent, Error> {
        Ok(LeafSchemaEvent {
            event_type: BubblegumEventTypeCodec.read(reader)?,
            version: VersionCodec.read(reader)?,
            schema: LeafSchemaCodec.read(reader)?,
            leaf_hash: FixedBytes::<32>.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Version {
    /// The `V1` variant.
    V1,
    /// The `V2` variant.
    V2,
}
/// Bounded codec for [`Version`].
#[derive(Clone, Copy, Debug)]
pub struct VersionCodec;
const VERSION_TAG_0: u8 = 0;
const VERSION_TAG_1: u8 = 1;
impl Encoder<Version> for VersionCodec {
    fn encoded_len(&self, value: &Version) -> Result<usize, Error> {
        let size = match value {
            Version::V1 => 0usize,
            Version::V2 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Version, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Version::V1 => {
                Number::<u8>::LE.write(&VERSION_TAG_0, writer)?;
                Ok(())
            }
            Version::V2 => {
                Number::<u8>::LE.write(&VERSION_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<Version> for VersionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Version, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            VERSION_TAG_0 => Ok(Version::V1),
            VERSION_TAG_1 => Ok(Version::V2),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenProgramVersion {
    /// The `Original` variant.
    Original,
    /// The `Token2022` variant.
    Token2022,
}
/// Bounded codec for [`TokenProgramVersion`].
#[derive(Clone, Copy, Debug)]
pub struct TokenProgramVersionCodec;
const TOKENPROGRAMVERSION_TAG_0: u8 = 0;
const TOKENPROGRAMVERSION_TAG_1: u8 = 1;
impl Encoder<TokenProgramVersion> for TokenProgramVersionCodec {
    fn encoded_len(&self, value: &TokenProgramVersion) -> Result<usize, Error> {
        let size = match value {
            TokenProgramVersion::Original => 0usize,
            TokenProgramVersion::Token2022 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &TokenProgramVersion, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TokenProgramVersion::Original => {
                Number::<u8>::LE.write(&TOKENPROGRAMVERSION_TAG_0, writer)?;
                Ok(())
            }
            TokenProgramVersion::Token2022 => {
                Number::<u8>::LE.write(&TOKENPROGRAMVERSION_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<TokenProgramVersion> for TokenProgramVersionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenProgramVersion, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            TOKENPROGRAMVERSION_TAG_0 => Ok(TokenProgramVersion::Original),
            TOKENPROGRAMVERSION_TAG_1 => Ok(TokenProgramVersion::Token2022),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenStandard {
    /// The `NonFungible` variant.
    NonFungible,
    /// The `FungibleAsset` variant.
    FungibleAsset,
    /// The `Fungible` variant.
    Fungible,
    /// The `NonFungibleEdition` variant.
    NonFungibleEdition,
}
/// Bounded codec for [`TokenStandard`].
#[derive(Clone, Copy, Debug)]
pub struct TokenStandardCodec;
const TOKENSTANDARD_TAG_0: u8 = 0;
const TOKENSTANDARD_TAG_1: u8 = 1;
const TOKENSTANDARD_TAG_2: u8 = 2;
const TOKENSTANDARD_TAG_3: u8 = 3;
impl Encoder<TokenStandard> for TokenStandardCodec {
    fn encoded_len(&self, value: &TokenStandard) -> Result<usize, Error> {
        let size = match value {
            TokenStandard::NonFungible => 0usize,
            TokenStandard::FungibleAsset => 0usize,
            TokenStandard::Fungible => 0usize,
            TokenStandard::NonFungibleEdition => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &TokenStandard, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TokenStandard::NonFungible => {
                Number::<u8>::LE.write(&TOKENSTANDARD_TAG_0, writer)?;
                Ok(())
            }
            TokenStandard::FungibleAsset => {
                Number::<u8>::LE.write(&TOKENSTANDARD_TAG_1, writer)?;
                Ok(())
            }
            TokenStandard::Fungible => {
                Number::<u8>::LE.write(&TOKENSTANDARD_TAG_2, writer)?;
                Ok(())
            }
            TokenStandard::NonFungibleEdition => {
                Number::<u8>::LE.write(&TOKENSTANDARD_TAG_3, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<TokenStandard> for TokenStandardCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenStandard, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            TOKENSTANDARD_TAG_0 => Ok(TokenStandard::NonFungible),
            TOKENSTANDARD_TAG_1 => Ok(TokenStandard::FungibleAsset),
            TOKENSTANDARD_TAG_2 => Ok(TokenStandard::Fungible),
            TOKENSTANDARD_TAG_3 => Ok(TokenStandard::NonFungibleEdition),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UseMethod {
    /// The `Burn` variant.
    Burn,
    /// The `Multiple` variant.
    Multiple,
    /// The `Single` variant.
    Single,
}
/// Bounded codec for [`UseMethod`].
#[derive(Clone, Copy, Debug)]
pub struct UseMethodCodec;
const USEMETHOD_TAG_0: u8 = 0;
const USEMETHOD_TAG_1: u8 = 1;
const USEMETHOD_TAG_2: u8 = 2;
impl Encoder<UseMethod> for UseMethodCodec {
    fn encoded_len(&self, value: &UseMethod) -> Result<usize, Error> {
        let size = match value {
            UseMethod::Burn => 0usize,
            UseMethod::Multiple => 0usize,
            UseMethod::Single => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UseMethod, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UseMethod::Burn => {
                Number::<u8>::LE.write(&USEMETHOD_TAG_0, writer)?;
                Ok(())
            }
            UseMethod::Multiple => {
                Number::<u8>::LE.write(&USEMETHOD_TAG_1, writer)?;
                Ok(())
            }
            UseMethod::Single => {
                Number::<u8>::LE.write(&USEMETHOD_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<UseMethod> for UseMethodCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UseMethod, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            USEMETHOD_TAG_0 => Ok(UseMethod::Burn),
            USEMETHOD_TAG_1 => Ok(UseMethod::Multiple),
            USEMETHOD_TAG_2 => Ok(UseMethod::Single),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecompressibleState {
    /// The `Enabled` variant.
    Enabled,
    /// The `Disabled` variant.
    Disabled,
}
/// Bounded codec for [`DecompressibleState`].
#[derive(Clone, Copy, Debug)]
pub struct DecompressibleStateCodec;
const DECOMPRESSIBLESTATE_TAG_0: u8 = 0;
const DECOMPRESSIBLESTATE_TAG_1: u8 = 1;
impl Encoder<DecompressibleState> for DecompressibleStateCodec {
    fn encoded_len(&self, value: &DecompressibleState) -> Result<usize, Error> {
        let size = match value {
            DecompressibleState::Enabled => 0usize,
            DecompressibleState::Disabled => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &DecompressibleState, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            DecompressibleState::Enabled => {
                Number::<u8>::LE.write(&DECOMPRESSIBLESTATE_TAG_0, writer)?;
                Ok(())
            }
            DecompressibleState::Disabled => {
                Number::<u8>::LE.write(&DECOMPRESSIBLESTATE_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<DecompressibleState> for DecompressibleStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DecompressibleState, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            DECOMPRESSIBLESTATE_TAG_0 => Ok(DecompressibleState::Enabled),
            DECOMPRESSIBLESTATE_TAG_1 => Ok(DecompressibleState::Disabled),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetDataSchema {
    /// The `Binary` variant.
    Binary,
    /// The `Json` variant.
    Json,
    /// The `MsgPack` variant.
    MsgPack,
}
/// Bounded codec for [`AssetDataSchema`].
#[derive(Clone, Copy, Debug)]
pub struct AssetDataSchemaCodec;
const ASSETDATASCHEMA_TAG_0: u8 = 0;
const ASSETDATASCHEMA_TAG_1: u8 = 1;
const ASSETDATASCHEMA_TAG_2: u8 = 2;
impl Encoder<AssetDataSchema> for AssetDataSchemaCodec {
    fn encoded_len(&self, value: &AssetDataSchema) -> Result<usize, Error> {
        let size = match value {
            AssetDataSchema::Binary => 0usize,
            AssetDataSchema::Json => 0usize,
            AssetDataSchema::MsgPack => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &AssetDataSchema, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            AssetDataSchema::Binary => {
                Number::<u8>::LE.write(&ASSETDATASCHEMA_TAG_0, writer)?;
                Ok(())
            }
            AssetDataSchema::Json => {
                Number::<u8>::LE.write(&ASSETDATASCHEMA_TAG_1, writer)?;
                Ok(())
            }
            AssetDataSchema::MsgPack => {
                Number::<u8>::LE.write(&ASSETDATASCHEMA_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<AssetDataSchema> for AssetDataSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AssetDataSchema, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            ASSETDATASCHEMA_TAG_0 => Ok(AssetDataSchema::Binary),
            ASSETDATASCHEMA_TAG_1 => Ok(AssetDataSchema::Json),
            ASSETDATASCHEMA_TAG_2 => Ok(AssetDataSchema::MsgPack),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BubblegumEventType {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `LeafSchemaEvent` variant.
    LeafSchemaEvent,
}
/// Bounded codec for [`BubblegumEventType`].
#[derive(Clone, Copy, Debug)]
pub struct BubblegumEventTypeCodec;
const BUBBLEGUMEVENTTYPE_TAG_0: u8 = 0;
const BUBBLEGUMEVENTTYPE_TAG_1: u8 = 1;
impl Encoder<BubblegumEventType> for BubblegumEventTypeCodec {
    fn encoded_len(&self, value: &BubblegumEventType) -> Result<usize, Error> {
        let size = match value {
            BubblegumEventType::Uninitialized => 0usize,
            BubblegumEventType::LeafSchemaEvent => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &BubblegumEventType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            BubblegumEventType::Uninitialized => {
                Number::<u8>::LE.write(&BUBBLEGUMEVENTTYPE_TAG_0, writer)?;
                Ok(())
            }
            BubblegumEventType::LeafSchemaEvent => {
                Number::<u8>::LE.write(&BUBBLEGUMEVENTTYPE_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<BubblegumEventType> for BubblegumEventTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BubblegumEventType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            BUBBLEGUMEVENTTYPE_TAG_0 => Ok(BubblegumEventType::Uninitialized),
            BUBBLEGUMEVENTTYPE_TAG_1 => Ok(BubblegumEventType::LeafSchemaEvent),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstructionName {
    /// The `Unknown` variant.
    Unknown,
    /// The `MintV1` variant.
    MintV1,
    /// The `Redeem` variant.
    Redeem,
    /// The `CancelRedeem` variant.
    CancelRedeem,
    /// The `Transfer` variant.
    Transfer,
    /// The `Delegate` variant.
    Delegate,
    /// The `DecompressV1` variant.
    DecompressV1,
    /// The `Compress` variant.
    Compress,
    /// The `Burn` variant.
    Burn,
    /// The `CreateTree` variant.
    CreateTree,
    /// The `VerifyCreator` variant.
    VerifyCreator,
    /// The `UnverifyCreator` variant.
    UnverifyCreator,
    /// The `VerifyCollection` variant.
    VerifyCollection,
    /// The `UnverifyCollection` variant.
    UnverifyCollection,
    /// The `SetAndVerifyCollection` variant.
    SetAndVerifyCollection,
    /// The `MintToCollectionV1` variant.
    MintToCollectionV1,
    /// The `SetDecompressibleState` variant.
    SetDecompressibleState,
    /// The `UpdateMetadata` variant.
    UpdateMetadata,
    /// The `BurnV2` variant.
    BurnV2,
    /// The `CollectV2` variant.
    CollectV2,
    /// The `CreateTreeV2` variant.
    CreateTreeV2,
    /// The `DelegateAndFreezeV2` variant.
    DelegateAndFreezeV2,
    /// The `DelegateV2` variant.
    DelegateV2,
    /// The `FreezeV2` variant.
    FreezeV2,
    /// The `MintV2` variant.
    MintV2,
    /// The `SetCollectionV2` variant.
    SetCollectionV2,
    /// The `SetNonTransferableV2` variant.
    SetNonTransferableV2,
    /// The `ThawAndRevokeV2` variant.
    ThawAndRevokeV2,
    /// The `ThawV2` variant.
    ThawV2,
    /// The `TransferV2` variant.
    TransferV2,
    /// The `UnverifyCreatorV2` variant.
    UnverifyCreatorV2,
    /// The `UpdateAssetDataV2` variant.
    UpdateAssetDataV2,
    /// The `UpdateMetadataV2` variant.
    UpdateMetadataV2,
    /// The `VerifyCreatorV2` variant.
    VerifyCreatorV2,
    /// The `CloseTreeV2` variant.
    CloseTreeV2,
}
/// Bounded codec for [`InstructionName`].
#[derive(Clone, Copy, Debug)]
pub struct InstructionNameCodec;
const INSTRUCTIONNAME_TAG_0: u8 = 0;
const INSTRUCTIONNAME_TAG_1: u8 = 1;
const INSTRUCTIONNAME_TAG_2: u8 = 2;
const INSTRUCTIONNAME_TAG_3: u8 = 3;
const INSTRUCTIONNAME_TAG_4: u8 = 4;
const INSTRUCTIONNAME_TAG_5: u8 = 5;
const INSTRUCTIONNAME_TAG_6: u8 = 6;
const INSTRUCTIONNAME_TAG_7: u8 = 7;
const INSTRUCTIONNAME_TAG_8: u8 = 8;
const INSTRUCTIONNAME_TAG_9: u8 = 9;
const INSTRUCTIONNAME_TAG_10: u8 = 10;
const INSTRUCTIONNAME_TAG_11: u8 = 11;
const INSTRUCTIONNAME_TAG_12: u8 = 12;
const INSTRUCTIONNAME_TAG_13: u8 = 13;
const INSTRUCTIONNAME_TAG_14: u8 = 14;
const INSTRUCTIONNAME_TAG_15: u8 = 15;
const INSTRUCTIONNAME_TAG_16: u8 = 16;
const INSTRUCTIONNAME_TAG_17: u8 = 17;
const INSTRUCTIONNAME_TAG_18: u8 = 18;
const INSTRUCTIONNAME_TAG_19: u8 = 19;
const INSTRUCTIONNAME_TAG_20: u8 = 20;
const INSTRUCTIONNAME_TAG_21: u8 = 21;
const INSTRUCTIONNAME_TAG_22: u8 = 22;
const INSTRUCTIONNAME_TAG_23: u8 = 23;
const INSTRUCTIONNAME_TAG_24: u8 = 24;
const INSTRUCTIONNAME_TAG_25: u8 = 25;
const INSTRUCTIONNAME_TAG_26: u8 = 26;
const INSTRUCTIONNAME_TAG_27: u8 = 27;
const INSTRUCTIONNAME_TAG_28: u8 = 28;
const INSTRUCTIONNAME_TAG_29: u8 = 29;
const INSTRUCTIONNAME_TAG_30: u8 = 30;
const INSTRUCTIONNAME_TAG_31: u8 = 31;
const INSTRUCTIONNAME_TAG_32: u8 = 32;
const INSTRUCTIONNAME_TAG_33: u8 = 33;
const INSTRUCTIONNAME_TAG_34: u8 = 34;
impl Encoder<InstructionName> for InstructionNameCodec {
    fn encoded_len(&self, value: &InstructionName) -> Result<usize, Error> {
        let size = match value {
            InstructionName::Unknown => 0usize,
            InstructionName::MintV1 => 0usize,
            InstructionName::Redeem => 0usize,
            InstructionName::CancelRedeem => 0usize,
            InstructionName::Transfer => 0usize,
            InstructionName::Delegate => 0usize,
            InstructionName::DecompressV1 => 0usize,
            InstructionName::Compress => 0usize,
            InstructionName::Burn => 0usize,
            InstructionName::CreateTree => 0usize,
            InstructionName::VerifyCreator => 0usize,
            InstructionName::UnverifyCreator => 0usize,
            InstructionName::VerifyCollection => 0usize,
            InstructionName::UnverifyCollection => 0usize,
            InstructionName::SetAndVerifyCollection => 0usize,
            InstructionName::MintToCollectionV1 => 0usize,
            InstructionName::SetDecompressibleState => 0usize,
            InstructionName::UpdateMetadata => 0usize,
            InstructionName::BurnV2 => 0usize,
            InstructionName::CollectV2 => 0usize,
            InstructionName::CreateTreeV2 => 0usize,
            InstructionName::DelegateAndFreezeV2 => 0usize,
            InstructionName::DelegateV2 => 0usize,
            InstructionName::FreezeV2 => 0usize,
            InstructionName::MintV2 => 0usize,
            InstructionName::SetCollectionV2 => 0usize,
            InstructionName::SetNonTransferableV2 => 0usize,
            InstructionName::ThawAndRevokeV2 => 0usize,
            InstructionName::ThawV2 => 0usize,
            InstructionName::TransferV2 => 0usize,
            InstructionName::UnverifyCreatorV2 => 0usize,
            InstructionName::UpdateAssetDataV2 => 0usize,
            InstructionName::UpdateMetadataV2 => 0usize,
            InstructionName::VerifyCreatorV2 => 0usize,
            InstructionName::CloseTreeV2 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &InstructionName, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            InstructionName::Unknown => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_0, writer)?;
                Ok(())
            }
            InstructionName::MintV1 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_1, writer)?;
                Ok(())
            }
            InstructionName::Redeem => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_2, writer)?;
                Ok(())
            }
            InstructionName::CancelRedeem => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_3, writer)?;
                Ok(())
            }
            InstructionName::Transfer => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_4, writer)?;
                Ok(())
            }
            InstructionName::Delegate => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_5, writer)?;
                Ok(())
            }
            InstructionName::DecompressV1 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_6, writer)?;
                Ok(())
            }
            InstructionName::Compress => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_7, writer)?;
                Ok(())
            }
            InstructionName::Burn => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_8, writer)?;
                Ok(())
            }
            InstructionName::CreateTree => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_9, writer)?;
                Ok(())
            }
            InstructionName::VerifyCreator => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_10, writer)?;
                Ok(())
            }
            InstructionName::UnverifyCreator => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_11, writer)?;
                Ok(())
            }
            InstructionName::VerifyCollection => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_12, writer)?;
                Ok(())
            }
            InstructionName::UnverifyCollection => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_13, writer)?;
                Ok(())
            }
            InstructionName::SetAndVerifyCollection => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_14, writer)?;
                Ok(())
            }
            InstructionName::MintToCollectionV1 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_15, writer)?;
                Ok(())
            }
            InstructionName::SetDecompressibleState => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_16, writer)?;
                Ok(())
            }
            InstructionName::UpdateMetadata => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_17, writer)?;
                Ok(())
            }
            InstructionName::BurnV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_18, writer)?;
                Ok(())
            }
            InstructionName::CollectV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_19, writer)?;
                Ok(())
            }
            InstructionName::CreateTreeV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_20, writer)?;
                Ok(())
            }
            InstructionName::DelegateAndFreezeV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_21, writer)?;
                Ok(())
            }
            InstructionName::DelegateV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_22, writer)?;
                Ok(())
            }
            InstructionName::FreezeV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_23, writer)?;
                Ok(())
            }
            InstructionName::MintV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_24, writer)?;
                Ok(())
            }
            InstructionName::SetCollectionV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_25, writer)?;
                Ok(())
            }
            InstructionName::SetNonTransferableV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_26, writer)?;
                Ok(())
            }
            InstructionName::ThawAndRevokeV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_27, writer)?;
                Ok(())
            }
            InstructionName::ThawV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_28, writer)?;
                Ok(())
            }
            InstructionName::TransferV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_29, writer)?;
                Ok(())
            }
            InstructionName::UnverifyCreatorV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_30, writer)?;
                Ok(())
            }
            InstructionName::UpdateAssetDataV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_31, writer)?;
                Ok(())
            }
            InstructionName::UpdateMetadataV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_32, writer)?;
                Ok(())
            }
            InstructionName::VerifyCreatorV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_33, writer)?;
                Ok(())
            }
            InstructionName::CloseTreeV2 => {
                Number::<u8>::LE.write(&INSTRUCTIONNAME_TAG_34, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<InstructionName> for InstructionNameCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InstructionName, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            INSTRUCTIONNAME_TAG_0 => Ok(InstructionName::Unknown),
            INSTRUCTIONNAME_TAG_1 => Ok(InstructionName::MintV1),
            INSTRUCTIONNAME_TAG_2 => Ok(InstructionName::Redeem),
            INSTRUCTIONNAME_TAG_3 => Ok(InstructionName::CancelRedeem),
            INSTRUCTIONNAME_TAG_4 => Ok(InstructionName::Transfer),
            INSTRUCTIONNAME_TAG_5 => Ok(InstructionName::Delegate),
            INSTRUCTIONNAME_TAG_6 => Ok(InstructionName::DecompressV1),
            INSTRUCTIONNAME_TAG_7 => Ok(InstructionName::Compress),
            INSTRUCTIONNAME_TAG_8 => Ok(InstructionName::Burn),
            INSTRUCTIONNAME_TAG_9 => Ok(InstructionName::CreateTree),
            INSTRUCTIONNAME_TAG_10 => Ok(InstructionName::VerifyCreator),
            INSTRUCTIONNAME_TAG_11 => Ok(InstructionName::UnverifyCreator),
            INSTRUCTIONNAME_TAG_12 => Ok(InstructionName::VerifyCollection),
            INSTRUCTIONNAME_TAG_13 => Ok(InstructionName::UnverifyCollection),
            INSTRUCTIONNAME_TAG_14 => Ok(InstructionName::SetAndVerifyCollection),
            INSTRUCTIONNAME_TAG_15 => Ok(InstructionName::MintToCollectionV1),
            INSTRUCTIONNAME_TAG_16 => Ok(InstructionName::SetDecompressibleState),
            INSTRUCTIONNAME_TAG_17 => Ok(InstructionName::UpdateMetadata),
            INSTRUCTIONNAME_TAG_18 => Ok(InstructionName::BurnV2),
            INSTRUCTIONNAME_TAG_19 => Ok(InstructionName::CollectV2),
            INSTRUCTIONNAME_TAG_20 => Ok(InstructionName::CreateTreeV2),
            INSTRUCTIONNAME_TAG_21 => Ok(InstructionName::DelegateAndFreezeV2),
            INSTRUCTIONNAME_TAG_22 => Ok(InstructionName::DelegateV2),
            INSTRUCTIONNAME_TAG_23 => Ok(InstructionName::FreezeV2),
            INSTRUCTIONNAME_TAG_24 => Ok(InstructionName::MintV2),
            INSTRUCTIONNAME_TAG_25 => Ok(InstructionName::SetCollectionV2),
            INSTRUCTIONNAME_TAG_26 => Ok(InstructionName::SetNonTransferableV2),
            INSTRUCTIONNAME_TAG_27 => Ok(InstructionName::ThawAndRevokeV2),
            INSTRUCTIONNAME_TAG_28 => Ok(InstructionName::ThawV2),
            INSTRUCTIONNAME_TAG_29 => Ok(InstructionName::TransferV2),
            INSTRUCTIONNAME_TAG_30 => Ok(InstructionName::UnverifyCreatorV2),
            INSTRUCTIONNAME_TAG_31 => Ok(InstructionName::UpdateAssetDataV2),
            INSTRUCTIONNAME_TAG_32 => Ok(InstructionName::UpdateMetadataV2),
            INSTRUCTIONNAME_TAG_33 => Ok(InstructionName::VerifyCreatorV2),
            INSTRUCTIONNAME_TAG_34 => Ok(InstructionName::CloseTreeV2),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `LeafSchemaV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeafSchemaV1Payload {
    /// The `id` wire field.
    pub id: Address,
    /// The `owner` wire field.
    pub owner: Address,
    /// The `delegate` wire field.
    pub delegate: Address,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
}
/// Bounded codec for [`LeafSchemaV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct LeafSchemaV1PayloadCodec;
impl Encoder<LeafSchemaV1Payload> for LeafSchemaV1PayloadCodec {
    fn encoded_len(&self, value: &LeafSchemaV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.id)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LeafSchemaV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.id, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegate, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Ok(())
    }
}
impl Decoder<LeafSchemaV1Payload> for LeafSchemaV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LeafSchemaV1Payload, Error> {
        Ok(LeafSchemaV1Payload {
            id: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            delegate: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
        })
    }
}
/// Validated data for `LeafSchemaV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeafSchemaV2Payload {
    /// The `id` wire field.
    pub id: Address,
    /// The `owner` wire field.
    pub owner: Address,
    /// The `delegate` wire field.
    pub delegate: Address,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `collection_hash` wire field.
    pub collection_hash: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: [u8; 32],
    /// The `flags` wire field.
    pub flags: u8,
}
/// Bounded codec for [`LeafSchemaV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct LeafSchemaV2PayloadCodec;
impl Encoder<LeafSchemaV2Payload> for LeafSchemaV2PayloadCodec {
    fn encoded_len(&self, value: &LeafSchemaV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.id)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.owner)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.collection_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LeafSchemaV2Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.id, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.owner, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegate, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        FixedBytes::<32>.write(&value.collection_hash, writer)?;
        FixedBytes::<32>.write(&value.asset_data_hash, writer)?;
        Number::<u8>::LE.write(&value.flags, writer)?;
        Ok(())
    }
}
impl Decoder<LeafSchemaV2Payload> for LeafSchemaV2PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LeafSchemaV2Payload, Error> {
        Ok(LeafSchemaV2Payload {
            id: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            owner: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            delegate: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            collection_hash: FixedBytes::<32>.read(reader)?,
            asset_data_hash: FixedBytes::<32>.read(reader)?,
            flags: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeafSchema {
    /// The `V1` variant.
    V1(LeafSchemaV1Payload),
    /// The `V2` variant.
    V2(LeafSchemaV2Payload),
}
/// Bounded codec for [`LeafSchema`].
#[derive(Clone, Copy, Debug)]
pub struct LeafSchemaCodec;
const LEAFSCHEMA_TAG_0: u8 = 0;
const LEAFSCHEMA_TAG_1: u8 = 1;
impl Encoder<LeafSchema> for LeafSchemaCodec {
    fn encoded_len(&self, value: &LeafSchema) -> Result<usize, Error> {
        let size = match value {
            LeafSchema::V1(value) => LeafSchemaV1PayloadCodec.encoded_len(value)?,
            LeafSchema::V2(value) => LeafSchemaV2PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &LeafSchema, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            LeafSchema::V1(value) => {
                Number::<u8>::LE.write(&LEAFSCHEMA_TAG_0, writer)?;
                LeafSchemaV1PayloadCodec.write(value, writer)
            }
            LeafSchema::V2(value) => {
                Number::<u8>::LE.write(&LEAFSCHEMA_TAG_1, writer)?;
                LeafSchemaV2PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<LeafSchema> for LeafSchemaCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LeafSchema, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            LEAFSCHEMA_TAG_0 => Ok(LeafSchema::V1(LeafSchemaV1PayloadCodec.read(reader)?)),
            LEAFSCHEMA_TAG_1 => Ok(LeafSchema::V2(LeafSchemaV2PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `TreeConfig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeConfig {
    /// The `tree_creator` wire field.
    pub tree_creator: Address,
    /// The `tree_delegate` wire field.
    pub tree_delegate: Address,
    /// The `total_mint_capacity` wire field.
    pub total_mint_capacity: u64,
    /// The `num_minted` wire field.
    pub num_minted: u64,
    /// The `is_public` wire field.
    pub is_public: bool,
    /// The `is_decompressible` wire field.
    pub is_decompressible: DecompressibleState,
    /// The `version` wire field.
    pub version: Version,
}
/// Bounded codec for [`TreeConfig`].
#[derive(Clone, Copy, Debug)]
pub struct TreeConfigCodec;
impl Encoder<TreeConfig> for TreeConfigCodec {
    fn encoded_len(&self, value: &TreeConfig) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[122, 245, 175, 248, 171, 34, 0, 207],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.tree_creator)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.tree_delegate)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.total_mint_capacity)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.num_minted)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_public)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(DecompressibleStateCodec.encoded_len(&value.is_decompressible)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(VersionCodec.encoded_len(&value.version)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TreeConfig, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[122, 245, 175, 248, 171, 34, 0, 207];
        let _record_start = writer.position();
        Affix {
            prefix: &[122, 245, 175, 248, 171, 34, 0, 207],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.tree_creator, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.tree_delegate, writer)?;
        Number::<u64>::LE.write(&value.total_mint_capacity, writer)?;
        Number::<u64>::LE.write(&value.num_minted, writer)?;
        Bool.write(&value.is_public, writer)?;
        DecompressibleStateCodec.write(&value.is_decompressible, writer)?;
        VersionCodec.write(&value.version, writer)?;
        let _record_bytes = writer
            .bytes()
            .get(_record_start..writer.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(())
    }
}
impl Decoder<TreeConfig> for TreeConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TreeConfig, Error> {
        const _DISCRIMINATOR: &[u8] = &[122, 245, 175, 248, 171, 34, 0, 207];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = TreeConfig {
            tree_creator: Affix {
                prefix: &[122, 245, 175, 248, 171, 34, 0, 207],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            tree_delegate: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            total_mint_capacity: Number::<u64>::LE.read(reader)?,
            num_minted: Number::<u64>::LE.read(reader)?,
            is_public: Bool.read(reader)?,
            is_decompressible: DecompressibleStateCodec.read(reader)?,
            version: VersionCodec.read(reader)?,
        };
        let _record_bytes = reader
            .bytes()
            .get(_record_start..reader.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(_record_value)
    }
}
/// Validated data for `Voucher`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Voucher {
    /// The `leaf_schema` wire field.
    pub leaf_schema: LeafSchema,
    /// The `index` wire field.
    pub index: u32,
    /// The `merkle_tree` wire field.
    pub merkle_tree: Address,
}
/// Bounded codec for [`Voucher`].
#[derive(Clone, Copy, Debug)]
pub struct VoucherCodec;
impl Encoder<Voucher> for VoucherCodec {
    fn encoded_len(&self, value: &Voucher) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[191, 204, 149, 234, 213, 165, 13, 65],
                    item: LeafSchemaCodec,
                    suffix: &[],
                }
                .encoded_len(&value.leaf_schema)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.merkle_tree)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Voucher, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[191, 204, 149, 234, 213, 165, 13, 65];
        let _record_start = writer.position();
        Affix {
            prefix: &[191, 204, 149, 234, 213, 165, 13, 65],
            item: LeafSchemaCodec,
            suffix: &[],
        }
        .write(&value.leaf_schema, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.merkle_tree, writer)?;
        let _record_bytes = writer
            .bytes()
            .get(_record_start..writer.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(())
    }
}
impl Decoder<Voucher> for VoucherCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Voucher, Error> {
        const _DISCRIMINATOR: &[u8] = &[191, 204, 149, 234, 213, 165, 13, 65];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Voucher {
            leaf_schema: Affix {
                prefix: &[191, 204, 149, 234, 213, 165, 13, 65],
                item: LeafSchemaCodec,
                suffix: &[],
            }
            .read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            merkle_tree: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        };
        let _record_bytes = reader
            .bytes()
            .get(_record_start..reader.position())
            .ok_or(Error::InvalidDiscriminator)?;
        if !_record_bytes.starts_with(_DISCRIMINATOR) {
            return Err(Error::InvalidDiscriminator);
        }
        Ok(_record_value)
    }
}
/// Validated data for `Burn`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Burn {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`Burn`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCodec;
impl Encoder<Burn> for BurnCodec {
    fn encoded_len(&self, value: &Burn) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Burn, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[116, 110, 29, 56, 107, 219, 42, 93]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<Burn> for BurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Burn, Error> {
        Constant(&[116, 110, 29, 56, 107, 219, 42, 93]).read(reader)?;
        Ok(Burn {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl Burn {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, None, false),
                /* leaf_delegate */ (false, None, false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[7..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = BurnCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        BurnCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BurnV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`BurnV2`].
#[derive(Clone, Copy, Debug)]
pub struct BurnV2Codec;
impl Encoder<BurnV2> for BurnV2Codec {
    fn encoded_len(&self, value: &BurnV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BurnV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[115, 210, 34, 240, 232, 143, 183, 16]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<BurnV2> for BurnV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnV2, Error> {
        Constant(&[115, 210, 34, 240, 232, 143, 183, 16]).read(reader)?;
        Ok(BurnV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl BurnV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 12 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (true, Some(false), true),
                /* mpl_core_cpi_signer */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* mpl_core_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[12..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = BurnV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        BurnV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CancelRedeem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CancelRedeem {
    /// The `root` wire field.
    pub root: [u8; 32],
}
/// Bounded codec for [`CancelRedeem`].
#[derive(Clone, Copy, Debug)]
pub struct CancelRedeemCodec;
impl Encoder<CancelRedeem> for CancelRedeemCodec {
    fn encoded_len(&self, value: &CancelRedeem) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CancelRedeem, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[111, 76, 232, 50, 39, 175, 48, 242]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        Ok(())
    }
}
impl Decoder<CancelRedeem> for CancelRedeemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CancelRedeem, Error> {
        Constant(&[111, 76, 232, 50, 39, 175, 48, 242]).read(reader)?;
        Ok(CancelRedeem {
            root: FixedBytes::<32>.read(reader)?,
        })
    }
}
impl CancelRedeem {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (true, Some(true), false),
                /* merkle_tree */ (true, Some(false), false),
                /* voucher */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[7..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CancelRedeemCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CancelRedeemCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseTreeV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseTreeV2 {}
/// Bounded codec for [`CloseTreeV2`].
#[derive(Clone, Copy, Debug)]
pub struct CloseTreeV2Codec;
impl Encoder<CloseTreeV2> for CloseTreeV2Codec {
    fn encoded_len(&self, value: &CloseTreeV2) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseTreeV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[45, 172, 6, 94, 28, 90, 157, 70]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseTreeV2> for CloseTreeV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseTreeV2, Error> {
        Constant(&[45, 172, 6, 94, 28, 90, 157, 70]).read(reader)?;
        let _ = reader;
        Ok(CloseTreeV2 {})
    }
}
impl CloseTreeV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* merkle_tree */ (true, Some(false), false),
                /* recipient */ (true, Some(false), false),
                /* fee_recipient */ (true, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CloseTreeV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CloseTreeV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CollectV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectV2 {}
/// Bounded codec for [`CollectV2`].
#[derive(Clone, Copy, Debug)]
pub struct CollectV2Codec;
impl Encoder<CollectV2> for CollectV2Codec {
    fn encoded_len(&self, value: &CollectV2) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CollectV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[21, 11, 159, 47, 4, 195, 106, 56]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CollectV2> for CollectV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectV2, Error> {
        Constant(&[21, 11, 159, 47, 4, 195, 106, 56]).read(reader)?;
        let _ = reader;
        Ok(CollectV2 {})
    }
}
impl CollectV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* destination */ (true, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CollectV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CollectV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Compress`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compress {}
/// Bounded codec for [`Compress`].
#[derive(Clone, Copy, Debug)]
pub struct CompressCodec;
impl Encoder<Compress> for CompressCodec {
    fn encoded_len(&self, value: &Compress) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Compress, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[82, 193, 176, 117, 176, 21, 115, 253]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Compress> for CompressCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Compress, Error> {
        Constant(&[82, 193, 176, 117, 176, 21, 115, 253]).read(reader)?;
        let _ = reader;
        Ok(Compress {})
    }
}
impl Compress {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 14 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(true), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (false, Some(false), false),
                /* token_account */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CompressCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CompressCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateTree`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateTree {
    /// The `max_depth` wire field.
    pub max_depth: u32,
    /// The `max_buffer_size` wire field.
    pub max_buffer_size: u32,
    /// The `public` wire field.
    pub public: Option<bool>,
}
/// Bounded codec for [`CreateTree`].
#[derive(Clone, Copy, Debug)]
pub struct CreateTreeCodec;
impl Encoder<CreateTree> for CreateTreeCodec {
    fn encoded_len(&self, value: &CreateTree) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_depth)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_buffer_size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.public)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateTree, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[165, 83, 136, 142, 89, 202, 47, 220]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.max_depth, writer)?;
        Number::<u32>::LE.write(&value.max_buffer_size, writer)?;
        OptionCodec::u8(Bool).write(&value.public, writer)?;
        Ok(())
    }
}
impl Decoder<CreateTree> for CreateTreeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateTree, Error> {
        Constant(&[165, 83, 136, 142, 89, 202, 47, 220]).read(reader)?;
        Ok(CreateTree {
            max_depth: Number::<u32>::LE.read(reader)?,
            max_buffer_size: Number::<u32>::LE.read(reader)?,
            public: OptionCodec::u8(Bool).read(reader)?,
        })
    }
}
impl CreateTree {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* tree_creator */ (false, Some(true), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CreateTreeCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CreateTreeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateTreeV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateTreeV2 {
    /// The `max_depth` wire field.
    pub max_depth: u32,
    /// The `max_buffer_size` wire field.
    pub max_buffer_size: u32,
    /// The `public` wire field.
    pub public: Option<bool>,
}
/// Bounded codec for [`CreateTreeV2`].
#[derive(Clone, Copy, Debug)]
pub struct CreateTreeV2Codec;
impl Encoder<CreateTreeV2> for CreateTreeV2Codec {
    fn encoded_len(&self, value: &CreateTreeV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_depth)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.max_buffer_size)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.public)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateTreeV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[55, 99, 95, 215, 142, 203, 227, 205]).write(&(), writer)?;
        Number::<u32>::LE.write(&value.max_depth, writer)?;
        Number::<u32>::LE.write(&value.max_buffer_size, writer)?;
        OptionCodec::u8(Bool).write(&value.public, writer)?;
        Ok(())
    }
}
impl Decoder<CreateTreeV2> for CreateTreeV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateTreeV2, Error> {
        Constant(&[55, 99, 95, 215, 142, 203, 227, 205]).read(reader)?;
        Ok(CreateTreeV2 {
            max_depth: Number::<u32>::LE.read(reader)?,
            max_buffer_size: Number::<u32>::LE.read(reader)?,
            public: OptionCodec::u8(Bool).read(reader)?,
        })
    }
}
impl CreateTreeV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* tree_creator */ (false, Some(true), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = CreateTreeV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        CreateTreeV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DecompressV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecompressV1 {
    /// The `metadata_args` wire field.
    pub metadata_args: MetadataArgs,
}
/// Bounded codec for [`DecompressV1`].
#[derive(Clone, Copy, Debug)]
pub struct DecompressV1Codec;
impl Encoder<DecompressV1> for DecompressV1Codec {
    fn encoded_len(&self, value: &DecompressV1) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.metadata_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DecompressV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[54, 85, 76, 70, 228, 250, 164, 81]).write(&(), writer)?;
        MetadataArgsCodec.write(&value.metadata_args, writer)?;
        Ok(())
    }
}
impl Decoder<DecompressV1> for DecompressV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DecompressV1, Error> {
        Constant(&[54, 85, 76, 70, 228, 250, 164, 81]).read(reader)?;
        Ok(DecompressV1 {
            metadata_args: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl DecompressV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 13 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* voucher */ (true, Some(false), false),
                /* leaf_owner */ (true, Some(true), false),
                /* token_account */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* mint_authority */ (true, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_rent */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* associated_token_program */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = DecompressV1Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        DecompressV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Delegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delegate {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`Delegate`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateCodec;
impl Encoder<Delegate> for DelegateCodec {
    fn encoded_len(&self, value: &Delegate) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Delegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[90, 147, 75, 178, 85, 88, 4, 137]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<Delegate> for DelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Delegate, Error> {
        Constant(&[90, 147, 75, 178, 85, 88, 4, 137]).read(reader)?;
        Ok(Delegate {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl Delegate {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(true), false),
                /* previous_leaf_delegate */ (false, Some(false), false),
                /* new_leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[8..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = DelegateCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        DelegateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DelegateAndFreezeV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateAndFreezeV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `collection_hash` wire field.
    pub collection_hash: Option<[u8; 32]>,
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`DelegateAndFreezeV2`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateAndFreezeV2Codec;
impl Encoder<DelegateAndFreezeV2> for DelegateAndFreezeV2Codec {
    fn encoded_len(&self, value: &DelegateAndFreezeV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.collection_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DelegateAndFreezeV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[17, 229, 35, 218, 190, 241, 250, 123]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.collection_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateAndFreezeV2> for DelegateAndFreezeV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateAndFreezeV2, Error> {
        Constant(&[17, 229, 35, 218, 190, 241, 250, 123]).read(reader)?;
        Ok(DelegateAndFreezeV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            collection_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl DelegateAndFreezeV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* leaf_owner */ (false, Some(true), true),
                /* previous_leaf_delegate */ (false, Some(false), true),
                /* new_leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[9..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = DelegateAndFreezeV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        DelegateAndFreezeV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DelegateV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `collection_hash` wire field.
    pub collection_hash: Option<[u8; 32]>,
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`DelegateV2`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateV2Codec;
impl Encoder<DelegateV2> for DelegateV2Codec {
    fn encoded_len(&self, value: &DelegateV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.collection_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DelegateV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[95, 87, 125, 140, 181, 131, 128, 227]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.collection_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateV2> for DelegateV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateV2, Error> {
        Constant(&[95, 87, 125, 140, 181, 131, 128, 227]).read(reader)?;
        Ok(DelegateV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            collection_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl DelegateV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* leaf_owner */ (false, Some(true), true),
                /* previous_leaf_delegate */ (false, Some(false), true),
                /* new_leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[9..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = DelegateV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        DelegateV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `FreezeV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreezeV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`FreezeV2`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeV2Codec;
impl Encoder<FreezeV2> for FreezeV2Codec {
    fn encoded_len(&self, value: &FreezeV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &FreezeV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[200, 151, 244, 102, 16, 195, 255, 3]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<FreezeV2> for FreezeV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeV2, Error> {
        Constant(&[200, 151, 244, 102, 16, 195, 255, 3]).read(reader)?;
        Ok(FreezeV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl FreezeV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[10..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = FreezeV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        FreezeV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintToCollectionV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintToCollectionV1 {
    /// The `metadata_args` wire field.
    pub metadata_args: MetadataArgs,
}
/// Bounded codec for [`MintToCollectionV1`].
#[derive(Clone, Copy, Debug)]
pub struct MintToCollectionV1Codec;
impl Encoder<MintToCollectionV1> for MintToCollectionV1Codec {
    fn encoded_len(&self, value: &MintToCollectionV1) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.metadata_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintToCollectionV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[153, 18, 178, 47, 197, 158, 86, 15]).write(&(), writer)?;
        MetadataArgsCodec.write(&value.metadata_args, writer)?;
        Ok(())
    }
}
impl Decoder<MintToCollectionV1> for MintToCollectionV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintToCollectionV1, Error> {
        Constant(&[153, 18, 178, 47, 197, 158, 86, 15]).read(reader)?;
        Ok(MintToCollectionV1 {
            metadata_args: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl MintToCollectionV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 16 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* tree_delegate */ (false, Some(true), false),
                /* collection_authority */ (false, Some(true), false),
                /* collection_authority_record_pda */ (false, Some(false), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection_metadata */ (true, Some(false), false),
                /* edition_account */ (false, Some(false), false),
                /* bubblegum_signer */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = MintToCollectionV1Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        MintToCollectionV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintV1 {
    /// The `message` wire field.
    pub message: MetadataArgs,
}
/// Bounded codec for [`MintV1`].
#[derive(Clone, Copy, Debug)]
pub struct MintV1Codec;
impl Encoder<MintV1> for MintV1Codec {
    fn encoded_len(&self, value: &MintV1) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[145, 98, 192, 118, 184, 147, 118, 104]).write(&(), writer)?;
        MetadataArgsCodec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<MintV1> for MintV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintV1, Error> {
        Constant(&[145, 98, 192, 118, 184, 147, 118, 104]).read(reader)?;
        Ok(MintV1 {
            message: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl MintV1 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* tree_delegate */ (false, Some(true), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = MintV1Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        MintV1Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintV2 {
    /// The `metadata_args` wire field.
    pub metadata_args: MetadataArgsV2,
    /// The `asset_data` wire field.
    pub asset_data: Option<Vec<u8>>,
    /// The `asset_data_schema` wire field.
    pub asset_data_schema: Option<AssetDataSchema>,
}
/// Bounded codec for [`MintV2`].
#[derive(Clone, Copy, Debug)]
pub struct MintV2Codec;
impl Encoder<MintV2> for MintV2Codec {
    fn encoded_len(&self, value: &MintV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(MetadataArgsV2Codec.encoded_len(&value.metadata_args)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }))
                .encoded_len(&value.asset_data)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AssetDataSchemaCodec).encoded_len(&value.asset_data_schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[120, 121, 23, 146, 173, 110, 199, 205]).write(&(), writer)?;
        MetadataArgsV2Codec.write(&value.metadata_args, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }))
        .write(&value.asset_data, writer)?;
        OptionCodec::u8(AssetDataSchemaCodec).write(&value.asset_data_schema, writer)?;
        Ok(())
    }
}
impl Decoder<MintV2> for MintV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintV2, Error> {
        Constant(&[120, 121, 23, 146, 173, 110, 199, 205]).read(reader)?;
        Ok(MintV2 {
            metadata_args: MetadataArgsV2Codec.read(reader)?,
            asset_data: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }))
            .read(reader)?,
            asset_data_schema: OptionCodec::u8(AssetDataSchemaCodec).read(reader)?,
        })
    }
}
impl MintV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 13 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* tree_delegate */ (false, Some(true), true),
                /* collection_authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (true, Some(false), true),
                /* mpl_core_cpi_signer */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* mpl_core_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = MintV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        MintV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Redeem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redeem {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`Redeem`].
#[derive(Clone, Copy, Debug)]
pub struct RedeemCodec;
impl Encoder<Redeem> for RedeemCodec {
    fn encoded_len(&self, value: &Redeem) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Redeem, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[184, 12, 86, 149, 70, 196, 97, 225]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<Redeem> for RedeemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Redeem, Error> {
        Constant(&[184, 12, 86, 149, 70, 196, 97, 225]).read(reader)?;
        Ok(Redeem {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl Redeem {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (true, Some(true), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* voucher */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[8..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = RedeemCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        RedeemCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetAndVerifyCollection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAndVerifyCollection {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgs,
    /// The `collection` wire field.
    pub collection: Address,
}
/// Bounded codec for [`SetAndVerifyCollection`].
#[derive(Clone, Copy, Debug)]
pub struct SetAndVerifyCollectionCodec;
impl Encoder<SetAndVerifyCollection> for SetAndVerifyCollectionCodec {
    fn encoded_len(&self, value: &SetAndVerifyCollection) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.collection)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetAndVerifyCollection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[235, 242, 121, 216, 158, 234, 180, 234]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsCodec.write(&value.message, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.collection, writer)?;
        Ok(())
    }
}
impl Decoder<SetAndVerifyCollection> for SetAndVerifyCollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAndVerifyCollection, Error> {
        Constant(&[235, 242, 121, 216, 158, 234, 180, 234]).read(reader)?;
        Ok(SetAndVerifyCollection {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsCodec.read(reader)?,
            collection: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
        })
    }
}
impl SetAndVerifyCollection {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 16 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* tree_delegate */ (false, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* collection_authority_record_pda */ (false, Some(false), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection_metadata */ (true, Some(false), false),
                /* edition_account */ (false, Some(false), false),
                /* bubblegum_signer */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[16..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetAndVerifyCollectionCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetAndVerifyCollectionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetCollectionV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetCollectionV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgsV2,
}
/// Bounded codec for [`SetCollectionV2`].
#[derive(Clone, Copy, Debug)]
pub struct SetCollectionV2Codec;
impl Encoder<SetCollectionV2> for SetCollectionV2Codec {
    fn encoded_len(&self, value: &SetCollectionV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsV2Codec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetCollectionV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[229, 35, 61, 91, 15, 14, 99, 160]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsV2Codec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<SetCollectionV2> for SetCollectionV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetCollectionV2, Error> {
        Constant(&[229, 35, 61, 91, 15, 14, 99, 160]).read(reader)?;
        Ok(SetCollectionV2 {
            root: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsV2Codec.read(reader)?,
        })
    }
}
impl SetCollectionV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 14 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* new_collection_authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (true, Some(false), true),
                /* new_core_collection */ (true, Some(false), true),
                /* mpl_core_cpi_signer */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* mpl_core_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[14..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetCollectionV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetCollectionV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetDecompressableState`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetDecompressableState {
    /// The `decompressable_state` wire field.
    pub decompressable_state: DecompressibleState,
}
/// Bounded codec for [`SetDecompressableState`].
#[derive(Clone, Copy, Debug)]
pub struct SetDecompressableStateCodec;
impl Encoder<SetDecompressableState> for SetDecompressableStateCodec {
    fn encoded_len(&self, value: &SetDecompressableState) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(DecompressibleStateCodec.encoded_len(&value.decompressable_state)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetDecompressableState, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[18, 135, 238, 168, 246, 195, 61, 115]).write(&(), writer)?;
        DecompressibleStateCodec.write(&value.decompressable_state, writer)?;
        Ok(())
    }
}
impl Decoder<SetDecompressableState> for SetDecompressableStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetDecompressableState, Error> {
        Constant(&[18, 135, 238, 168, 246, 195, 61, 115]).read(reader)?;
        Ok(SetDecompressableState {
            decompressable_state: DecompressibleStateCodec.read(reader)?,
        })
    }
}
impl SetDecompressableState {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* tree_creator */ (false, Some(true), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetDecompressableStateCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetDecompressableStateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetDecompressibleState`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetDecompressibleState {
    /// The `decompressable_state` wire field.
    pub decompressable_state: DecompressibleState,
}
/// Bounded codec for [`SetDecompressibleState`].
#[derive(Clone, Copy, Debug)]
pub struct SetDecompressibleStateCodec;
impl Encoder<SetDecompressibleState> for SetDecompressibleStateCodec {
    fn encoded_len(&self, value: &SetDecompressibleState) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(DecompressibleStateCodec.encoded_len(&value.decompressable_state)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetDecompressibleState, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[82, 104, 152, 6, 149, 111, 100, 13]).write(&(), writer)?;
        DecompressibleStateCodec.write(&value.decompressable_state, writer)?;
        Ok(())
    }
}
impl Decoder<SetDecompressibleState> for SetDecompressibleStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetDecompressibleState, Error> {
        Constant(&[82, 104, 152, 6, 149, 111, 100, 13]).read(reader)?;
        Ok(SetDecompressibleState {
            decompressable_state: DecompressibleStateCodec.read(reader)?,
        })
    }
}
impl SetDecompressibleState {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 2 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* tree_creator */ (false, Some(true), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetDecompressibleStateCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetDecompressibleStateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetNonTransferableV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetNonTransferableV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`SetNonTransferableV2`].
#[derive(Clone, Copy, Debug)]
pub struct SetNonTransferableV2Codec;
impl Encoder<SetNonTransferableV2> for SetNonTransferableV2Codec {
    fn encoded_len(&self, value: &SetNonTransferableV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetNonTransferableV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[181, 141, 206, 58, 242, 199, 152, 168]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<SetNonTransferableV2> for SetNonTransferableV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetNonTransferableV2, Error> {
        Constant(&[181, 141, 206, 58, 242, 199, 152, 168]).read(reader)?;
        Ok(SetNonTransferableV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl SetNonTransferableV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[10..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetNonTransferableV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetNonTransferableV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetTreeDelegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetTreeDelegate {}
/// Bounded codec for [`SetTreeDelegate`].
#[derive(Clone, Copy, Debug)]
pub struct SetTreeDelegateCodec;
impl Encoder<SetTreeDelegate> for SetTreeDelegateCodec {
    fn encoded_len(&self, value: &SetTreeDelegate) -> Result<usize, Error> {
        let size = 8usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SetTreeDelegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[253, 118, 66, 37, 190, 49, 154, 102]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SetTreeDelegate> for SetTreeDelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetTreeDelegate, Error> {
        Constant(&[253, 118, 66, 37, 190, 49, 154, 102]).read(reader)?;
        let _ = reader;
        Ok(SetTreeDelegate {})
    }
}
impl SetTreeDelegate {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 5 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* tree_creator */ (false, Some(true), false),
                /* new_tree_delegate */ (false, Some(false), false),
                /* merkle_tree */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = SetTreeDelegateCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        SetTreeDelegateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ThawAndRevokeV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThawAndRevokeV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `collection_hash` wire field.
    pub collection_hash: Option<[u8; 32]>,
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`ThawAndRevokeV2`].
#[derive(Clone, Copy, Debug)]
pub struct ThawAndRevokeV2Codec;
impl Encoder<ThawAndRevokeV2> for ThawAndRevokeV2Codec {
    fn encoded_len(&self, value: &ThawAndRevokeV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.collection_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ThawAndRevokeV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[86, 214, 190, 37, 167, 4, 28, 116]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.collection_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<ThawAndRevokeV2> for ThawAndRevokeV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ThawAndRevokeV2, Error> {
        Constant(&[86, 214, 190, 37, 167, 4, 28, 116]).read(reader)?;
        Ok(ThawAndRevokeV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            collection_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl ThawAndRevokeV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* leaf_delegate */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[8..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = ThawAndRevokeV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        ThawAndRevokeV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ThawV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThawV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`ThawV2`].
#[derive(Clone, Copy, Debug)]
pub struct ThawV2Codec;
impl Encoder<ThawV2> for ThawV2Codec {
    fn encoded_len(&self, value: &ThawV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ThawV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[96, 133, 101, 93, 82, 220, 146, 191]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<ThawV2> for ThawV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ThawV2, Error> {
        Constant(&[96, 133, 101, 93, 82, 220, 146, 191]).read(reader)?;
        Ok(ThawV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl ThawV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[10..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = ThawV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        ThawV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Transfer`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`Transfer`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCodec;
impl Encoder<Transfer> for TransferCodec {
    fn encoded_len(&self, value: &Transfer) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Transfer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[163, 52, 200, 231, 140, 3, 69, 186]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<Transfer> for TransferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Transfer, Error> {
        Constant(&[163, 52, 200, 231, 140, 3, 69, 186]).read(reader)?;
        Ok(Transfer {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl Transfer {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, None, false),
                /* leaf_delegate */ (false, None, false),
                /* new_leaf_owner */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[8..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = TransferCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        TransferCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
}
/// Bounded codec for [`TransferV2`].
#[derive(Clone, Copy, Debug)]
pub struct TransferV2Codec;
impl Encoder<TransferV2> for TransferV2Codec {
    fn encoded_len(&self, value: &TransferV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[119, 40, 6, 235, 234, 221, 248, 49]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        Ok(())
    }
}
impl Decoder<TransferV2> for TransferV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferV2, Error> {
        Constant(&[119, 40, 6, 235, 234, 221, 248, 49]).read(reader)?;
        Ok(TransferV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
        })
    }
}
impl TransferV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 11 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* new_leaf_owner */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[11..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = TransferV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        TransferV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UnverifyCollection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnverifyCollection {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgs,
}
/// Bounded codec for [`UnverifyCollection`].
#[derive(Clone, Copy, Debug)]
pub struct UnverifyCollectionCodec;
impl Encoder<UnverifyCollection> for UnverifyCollectionCodec {
    fn encoded_len(&self, value: &UnverifyCollection) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnverifyCollection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[250, 251, 42, 106, 41, 137, 186, 168]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsCodec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<UnverifyCollection> for UnverifyCollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnverifyCollection, Error> {
        Constant(&[250, 251, 42, 106, 41, 137, 186, 168]).read(reader)?;
        Ok(UnverifyCollection {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl UnverifyCollection {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 16 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* tree_delegate */ (false, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* collection_authority_record_pda */ (false, Some(false), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection_metadata */ (true, Some(false), false),
                /* edition_account */ (false, Some(false), false),
                /* bubblegum_signer */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[16..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UnverifyCollectionCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UnverifyCollectionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UnverifyCreator`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnverifyCreator {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgs,
}
/// Bounded codec for [`UnverifyCreator`].
#[derive(Clone, Copy, Debug)]
pub struct UnverifyCreatorCodec;
impl Encoder<UnverifyCreator> for UnverifyCreatorCodec {
    fn encoded_len(&self, value: &UnverifyCreator) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnverifyCreator, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[107, 178, 57, 39, 105, 115, 112, 152]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsCodec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<UnverifyCreator> for UnverifyCreatorCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnverifyCreator, Error> {
        Constant(&[107, 178, 57, 39, 105, 115, 112, 152]).read(reader)?;
        Ok(UnverifyCreator {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl UnverifyCreator {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* creator */ (false, Some(true), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[9..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UnverifyCreatorCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UnverifyCreatorCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UnverifyCreatorV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnverifyCreatorV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgsV2,
}
/// Bounded codec for [`UnverifyCreatorV2`].
#[derive(Clone, Copy, Debug)]
pub struct UnverifyCreatorV2Codec;
impl Encoder<UnverifyCreatorV2> for UnverifyCreatorV2Codec {
    fn encoded_len(&self, value: &UnverifyCreatorV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsV2Codec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnverifyCreatorV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[174, 112, 29, 142, 230, 100, 239, 7]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsV2Codec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<UnverifyCreatorV2> for UnverifyCreatorV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnverifyCreatorV2, Error> {
        Constant(&[174, 112, 29, 142, 230, 100, 239, 7]).read(reader)?;
        Ok(UnverifyCreatorV2 {
            root: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsV2Codec.read(reader)?,
        })
    }
}
impl UnverifyCreatorV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* creator */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[9..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UnverifyCreatorV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UnverifyCreatorV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateAssetDataV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateAssetDataV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `previous_asset_data_hash` wire field.
    pub previous_asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `new_asset_data` wire field.
    pub new_asset_data: Option<Vec<u8>>,
    /// The `new_asset_data_schema` wire field.
    pub new_asset_data_schema: Option<AssetDataSchema>,
}
/// Bounded codec for [`UpdateAssetDataV2`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateAssetDataV2Codec;
impl Encoder<UpdateAssetDataV2> for UpdateAssetDataV2Codec {
    fn encoded_len(&self, value: &UpdateAssetDataV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.previous_asset_data_hash)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }))
                .encoded_len(&value.new_asset_data)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AssetDataSchemaCodec).encoded_len(&value.new_asset_data_schema)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateAssetDataV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[59, 56, 111, 43, 95, 14, 11, 61]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.previous_asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Number::<u8>::LE,
            minimum: 1,
        }))
        .write(&value.new_asset_data, writer)?;
        OptionCodec::u8(AssetDataSchemaCodec).write(&value.new_asset_data_schema, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateAssetDataV2> for UpdateAssetDataV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateAssetDataV2, Error> {
        Constant(&[59, 56, 111, 43, 95, 14, 11, 61]).read(reader)?;
        Ok(UpdateAssetDataV2 {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            previous_asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            new_asset_data: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }))
            .read(reader)?,
            new_asset_data_schema: OptionCodec::u8(AssetDataSchemaCodec).read(reader)?,
        })
    }
}
impl UpdateAssetDataV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[10..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UpdateAssetDataV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UpdateAssetDataV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateMetadata`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateMetadata {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `current_metadata` wire field.
    pub current_metadata: MetadataArgs,
    /// The `update_args` wire field.
    pub update_args: UpdateArgs,
}
/// Bounded codec for [`UpdateMetadata`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateMetadataCodec;
impl Encoder<UpdateMetadata> for UpdateMetadataCodec {
    fn encoded_len(&self, value: &UpdateMetadata) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.current_metadata)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(UpdateArgsCodec.encoded_len(&value.update_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateMetadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[170, 182, 43, 239, 97, 78, 225, 186]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsCodec.write(&value.current_metadata, writer)?;
        UpdateArgsCodec.write(&value.update_args, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateMetadata> for UpdateMetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateMetadata, Error> {
        Constant(&[170, 182, 43, 239, 97, 78, 225, 186]).read(reader)?;
        Ok(UpdateMetadata {
            root: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            current_metadata: MetadataArgsCodec.read(reader)?,
            update_args: UpdateArgsCodec.read(reader)?,
        })
    }
}
impl UpdateMetadata {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 13 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* collection_mint */ (false, Some(false), true),
                /* collection_metadata */ (false, Some(false), true),
                /* collection_authority_record_pda */ (false, Some(false), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[13..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UpdateMetadataCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UpdateMetadataCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateMetadataV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateMetadataV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `current_metadata` wire field.
    pub current_metadata: MetadataArgsV2,
    /// The `update_args` wire field.
    pub update_args: UpdateArgs,
}
/// Bounded codec for [`UpdateMetadataV2`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateMetadataV2Codec;
impl Encoder<UpdateMetadataV2> for UpdateMetadataV2Codec {
    fn encoded_len(&self, value: &UpdateMetadataV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsV2Codec.encoded_len(&value.current_metadata)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(UpdateArgsCodec.encoded_len(&value.update_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateMetadataV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[43, 103, 89, 42, 121, 242, 62, 72]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsV2Codec.write(&value.current_metadata, writer)?;
        UpdateArgsCodec.write(&value.update_args, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateMetadataV2> for UpdateMetadataV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateMetadataV2, Error> {
        Constant(&[43, 103, 89, 42, 121, 242, 62, 72]).read(reader)?;
        Ok(UpdateMetadataV2 {
            root: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            current_metadata: MetadataArgsV2Codec.read(reader)?,
            update_args: UpdateArgsCodec.read(reader)?,
        })
    }
}
impl UpdateMetadataV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* core_collection */ (false, Some(false), true),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[10..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = UpdateMetadataV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        UpdateMetadataV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VerifyCollection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyCollection {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgs,
}
/// Bounded codec for [`VerifyCollection`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyCollectionCodec;
impl Encoder<VerifyCollection> for VerifyCollectionCodec {
    fn encoded_len(&self, value: &VerifyCollection) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VerifyCollection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[56, 113, 101, 253, 79, 55, 122, 169]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsCodec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<VerifyCollection> for VerifyCollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifyCollection, Error> {
        Constant(&[56, 113, 101, 253, 79, 55, 122, 169]).read(reader)?;
        Ok(VerifyCollection {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl VerifyCollection {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 16 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* tree_delegate */ (false, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* collection_authority_record_pda */ (false, Some(false), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection_metadata */ (true, Some(false), false),
                /* edition_account */ (false, Some(false), false),
                /* bubblegum_signer */ (false, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* token_metadata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[16..]
            .iter()
            .any(|meta| meta.role != remaining_role)
        {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = VerifyCollectionCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        VerifyCollectionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VerifyCreator`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyCreator {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `data_hash` wire field.
    pub data_hash: [u8; 32],
    /// The `creator_hash` wire field.
    pub creator_hash: [u8; 32],
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgs,
}
/// Bounded codec for [`VerifyCreator`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyCreatorCodec;
impl Encoder<VerifyCreator> for VerifyCreatorCodec {
    fn encoded_len(&self, value: &VerifyCreator) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.creator_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsCodec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VerifyCreator, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[52, 17, 96, 132, 71, 4, 85, 194]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        FixedBytes::<32>.write(&value.data_hash, writer)?;
        FixedBytes::<32>.write(&value.creator_hash, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsCodec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<VerifyCreator> for VerifyCreatorCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifyCreator, Error> {
        Constant(&[52, 17, 96, 132, 71, 4, 85, 194]).read(reader)?;
        Ok(VerifyCreator {
            root: FixedBytes::<32>.read(reader)?,
            data_hash: FixedBytes::<32>.read(reader)?,
            creator_hash: FixedBytes::<32>.read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsCodec.read(reader)?,
        })
    }
}
impl VerifyCreator {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (false, Some(false), false),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), false),
                /* merkle_tree */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* creator */ (false, Some(true), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[9..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = VerifyCreatorCodec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        VerifyCreatorCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VerifyCreatorV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyCreatorV2 {
    /// The `root` wire field.
    pub root: [u8; 32],
    /// The `asset_data_hash` wire field.
    pub asset_data_hash: Option<[u8; 32]>,
    /// The `flags` wire field.
    pub flags: Option<u8>,
    /// The `nonce` wire field.
    pub nonce: u64,
    /// The `index` wire field.
    pub index: u32,
    /// The `message` wire field.
    pub message: MetadataArgsV2,
}
/// Bounded codec for [`VerifyCreatorV2`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyCreatorV2Codec;
impl Encoder<VerifyCreatorV2> for VerifyCreatorV2Codec {
    fn encoded_len(&self, value: &VerifyCreatorV2) -> Result<usize, Error> {
        let size = 8usize;
        let size = size
            .checked_add(FixedBytes::<32>.encoded_len(&value.root)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(FixedBytes::<32>).encoded_len(&value.asset_data_hash)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.flags)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.nonce)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u32>::LE.encoded_len(&value.index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(MetadataArgsV2Codec.encoded_len(&value.message)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &VerifyCreatorV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[85, 138, 140, 42, 22, 241, 118, 102]).write(&(), writer)?;
        FixedBytes::<32>.write(&value.root, writer)?;
        OptionCodec::u8(FixedBytes::<32>).write(&value.asset_data_hash, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.flags, writer)?;
        Number::<u64>::LE.write(&value.nonce, writer)?;
        Number::<u32>::LE.write(&value.index, writer)?;
        MetadataArgsV2Codec.write(&value.message, writer)?;
        Ok(())
    }
}
impl Decoder<VerifyCreatorV2> for VerifyCreatorV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifyCreatorV2, Error> {
        Constant(&[85, 138, 140, 42, 22, 241, 118, 102]).read(reader)?;
        Ok(VerifyCreatorV2 {
            root: FixedBytes::<32>.read(reader)?,
            asset_data_hash: OptionCodec::u8(FixedBytes::<32>).read(reader)?,
            flags: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            nonce: Number::<u64>::LE.read(reader)?,
            index: Number::<u32>::LE.read(reader)?,
            message: MetadataArgsV2Codec.read(reader)?,
        })
    }
}
impl VerifyCreatorV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* tree_authority */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* creator */ (false, Some(true), true),
                /* leaf_owner */ (false, Some(false), false),
                /* leaf_delegate */ (false, Some(false), true),
                /* merkle_tree */ (true, Some(false), false),
                /* log_wrapper */ (false, Some(false), false),
                /* compression_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
            ])
            .all(|(meta, expected)| role_matches(program, meta, &expected))
        {
            return Err(Error::InvalidValue);
        }
        let remaining_role = AccountRole::from_privileges(false, false);

        if accounts[9..].iter().any(|meta| meta.role != remaining_role) {
            return Err(Error::InvalidValue);
        }
        Ok(())
    }
    /// Builds an invocation preserving ordered accounts and duplicates.
    /// All role addresses and defaults are explicit caller inputs.
    pub fn instruction(
        &self,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        self.instruction_with_program(PROGRAM, accounts, limits)
    }
    /// Builds an invocation for an explicit program identity with the same wire contract.
    pub fn instruction_with_program(
        &self,
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        Self::validate_accounts(program, accounts, limits)?;
        let data = VerifyCreatorV2Codec.encode(self, limits.max_bytes)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(accounts.len())
            .map_err(|_| Error::AllocationFailed)?;
        owned.extend_from_slice(accounts);
        Ok(Instruction {
            program_address: program,
            accounts: owned,
            data,
        })
    }
    /// Strictly parses data, identity, ordered privileges and remaining accounts.
    pub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {
        Self::parse_with_program(PROGRAM, instruction, limits)
    }
    /// Strictly parses an invocation against an explicit expected program identity.
    pub fn parse_with_program(
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<Self, Error> {
        if instruction.program_address != program {
            return Err(Error::InvalidValue);
        }

        Self::validate_accounts(program, &instruction.accounts, limits)?;
        VerifyCreatorV2Codec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MplBubblegumError {
    /// The `AssetOwnerMismatch` program error.
    AssetOwnerMismatch,
    /// The `PublicKeyMismatch` program error.
    PublicKeyMismatch,
    /// The `HashingMismatch` program error.
    HashingMismatch,
    /// The `UnsupportedSchemaVersion` program error.
    UnsupportedSchemaVersion,
    /// The `CreatorShareSum` program error.
    CreatorShareSum,
    /// The `DuplicateCreatorAddress` program error.
    DuplicateCreatorAddress,
    /// The `CreatorNotVerified` program error.
    CreatorNotVerified,
    /// The `CreatorNotFound` program error.
    CreatorNotFound,
    /// The `NoCreators` program error.
    NoCreators,
    /// The `CreatorHashMismatch` program error.
    CreatorHashMismatch,
    /// The `DataHashMismatch` program error.
    DataHashMismatch,
    /// The `CreatorsTooLong` program error.
    CreatorsTooLong,
    /// The `NameTooLong` program error.
    NameTooLong,
    /// The `SymbolTooLong` program error.
    SymbolTooLong,
    /// The `UriTooLong` program error.
    UriTooLong,
    /// The `BasisPointsExceedMax` program error.
    BasisPointsExceedMax,
    /// The `TreeCreatorOrDelegateMustSign` program error.
    TreeCreatorOrDelegateMustSign,
    /// The `InsufficientMintApprovals` program error.
    InsufficientMintApprovals,
    /// The `NumericalOverflow` program error.
    NumericalOverflow,
    /// The `IncorrectOwner` program error.
    IncorrectOwner,
    /// The `CannotVerifyCollection` program error.
    CannotVerifyCollection,
    /// The `CollectionNotFound` program error.
    CollectionNotFound,
    /// The `CollectionAlreadyVerified` program error.
    CollectionAlreadyVerified,
    /// The `CollectionAlreadyUnverified` program error.
    CollectionAlreadyUnverified,
    /// The `IncorrectMetadataUpdateAuthority` program error.
    IncorrectMetadataUpdateAuthority,
    /// The `OwnerOrDelegateMismatch` program error.
    OwnerOrDelegateMismatch,
    /// The `CollectionMustBeSized` program error.
    CollectionMustBeSized,
    /// The `MetadataMintMismatch` program error.
    MetadataMintMismatch,
    /// The `InvalidCollectionAuthority` program error.
    InvalidCollectionAuthority,
    /// The `InvalidDelegateRecordPda` program error.
    InvalidDelegateRecordPda,
    /// The `EditionMismatch` program error.
    EditionMismatch,
    /// The `CollectionMustBeMasterEdition` program error.
    CollectionMustBeMasterEdition,
    /// The `ExternalErrorConversion` program error.
    ExternalErrorConversion,
    /// The `DecompressionDisabled` program error.
    DecompressionDisabled,
    /// The `MissingCollectionMint` program error.
    MissingCollectionMint,
    /// The `MissingCollectionMetadata` program error.
    MissingCollectionMetadata,
    /// The `CollectionMismatch` program error.
    CollectionMismatch,
    /// The `MetadataNotMutable` program error.
    MetadataNotMutable,
    /// The `PrimarySaleCanOnlyBeFlipped` program error.
    PrimarySaleCanOnlyBeFlipped,
    /// The `CreatorDidNotUnverify` program error.
    CreatorDidNotUnverify,
    /// The `OnlyNonFungibleSupported` program error.
    OnlyNonFungibleSupported,
    /// The `CanopySizeTooSmall` program error.
    CanopySizeTooSmall,
    /// The `InvalidLogWrapper` program error.
    InvalidLogWrapper,
    /// The `InvalidCompressionProgram` program error.
    InvalidCompressionProgram,
    /// The `LeafMustBeDelegated` program error.
    LeafMustBeDelegated,
    /// The `AssetFrozen` program error.
    AssetFrozen,
    /// The `AssetNonTransferable` program error.
    AssetNonTransferable,
    /// The `InvalidAuthority` program error.
    InvalidAuthority,
    /// The `CollectionFrozen` program error.
    CollectionFrozen,
    /// The `CoreCollectionsMustHavePlugin` program error.
    CoreCollectionsMustHavePlugin,
    /// The `FeatureNotAvailable` program error.
    FeatureNotAvailable,
    /// The `MissingCollectionAccount` program error.
    MissingCollectionAccount,
    /// The `AssetDataTooLong` program error.
    AssetDataTooLong,
    /// The `AlreadyInCollection` program error.
    AlreadyInCollection,
    /// The `AlreadyNotInCollection` program error.
    AlreadyNotInCollection,
    /// The `MissingMplCoreCpiSigner` program error.
    MissingMplCoreCpiSigner,
    /// The `AssetNotFrozen` program error.
    AssetNotFrozen,
    /// The `CollectionMustHaveRoyaltiesPlugin` program error.
    CollectionMustHaveRoyaltiesPlugin,
    /// The `InheritedSellerFeeCannotHaveLeafCreators` program error.
    InheritedSellerFeeCannotHaveLeafCreators,
    /// The `CannotRemoveFromCollectionWithInheritedSellerFee` program error.
    CannotRemoveFromCollectionWithInheritedSellerFee,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl MplBubblegumError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_6000 => Self::AssetOwnerMismatch,
            PROGRAM_ERROR_6001 => Self::PublicKeyMismatch,
            PROGRAM_ERROR_6002 => Self::HashingMismatch,
            PROGRAM_ERROR_6003 => Self::UnsupportedSchemaVersion,
            PROGRAM_ERROR_6004 => Self::CreatorShareSum,
            PROGRAM_ERROR_6005 => Self::DuplicateCreatorAddress,
            PROGRAM_ERROR_6006 => Self::CreatorNotVerified,
            PROGRAM_ERROR_6007 => Self::CreatorNotFound,
            PROGRAM_ERROR_6008 => Self::NoCreators,
            PROGRAM_ERROR_6009 => Self::CreatorHashMismatch,
            PROGRAM_ERROR_6010 => Self::DataHashMismatch,
            PROGRAM_ERROR_6011 => Self::CreatorsTooLong,
            PROGRAM_ERROR_6012 => Self::NameTooLong,
            PROGRAM_ERROR_6013 => Self::SymbolTooLong,
            PROGRAM_ERROR_6014 => Self::UriTooLong,
            PROGRAM_ERROR_6015 => Self::BasisPointsExceedMax,
            PROGRAM_ERROR_6016 => Self::TreeCreatorOrDelegateMustSign,
            PROGRAM_ERROR_6017 => Self::InsufficientMintApprovals,
            PROGRAM_ERROR_6018 => Self::NumericalOverflow,
            PROGRAM_ERROR_6019 => Self::IncorrectOwner,
            PROGRAM_ERROR_6020 => Self::CannotVerifyCollection,
            PROGRAM_ERROR_6021 => Self::CollectionNotFound,
            PROGRAM_ERROR_6022 => Self::CollectionAlreadyVerified,
            PROGRAM_ERROR_6023 => Self::CollectionAlreadyUnverified,
            PROGRAM_ERROR_6024 => Self::IncorrectMetadataUpdateAuthority,
            PROGRAM_ERROR_6025 => Self::OwnerOrDelegateMismatch,
            PROGRAM_ERROR_6026 => Self::CollectionMustBeSized,
            PROGRAM_ERROR_6027 => Self::MetadataMintMismatch,
            PROGRAM_ERROR_6028 => Self::InvalidCollectionAuthority,
            PROGRAM_ERROR_6029 => Self::InvalidDelegateRecordPda,
            PROGRAM_ERROR_6030 => Self::EditionMismatch,
            PROGRAM_ERROR_6031 => Self::CollectionMustBeMasterEdition,
            PROGRAM_ERROR_6032 => Self::ExternalErrorConversion,
            PROGRAM_ERROR_6033 => Self::DecompressionDisabled,
            PROGRAM_ERROR_6034 => Self::MissingCollectionMint,
            PROGRAM_ERROR_6035 => Self::MissingCollectionMetadata,
            PROGRAM_ERROR_6036 => Self::CollectionMismatch,
            PROGRAM_ERROR_6037 => Self::MetadataNotMutable,
            PROGRAM_ERROR_6038 => Self::PrimarySaleCanOnlyBeFlipped,
            PROGRAM_ERROR_6039 => Self::CreatorDidNotUnverify,
            PROGRAM_ERROR_6040 => Self::OnlyNonFungibleSupported,
            PROGRAM_ERROR_6041 => Self::CanopySizeTooSmall,
            PROGRAM_ERROR_6042 => Self::InvalidLogWrapper,
            PROGRAM_ERROR_6043 => Self::InvalidCompressionProgram,
            PROGRAM_ERROR_6044 => Self::LeafMustBeDelegated,
            PROGRAM_ERROR_6045 => Self::AssetFrozen,
            PROGRAM_ERROR_6046 => Self::AssetNonTransferable,
            PROGRAM_ERROR_6047 => Self::InvalidAuthority,
            PROGRAM_ERROR_6048 => Self::CollectionFrozen,
            PROGRAM_ERROR_6049 => Self::CoreCollectionsMustHavePlugin,
            PROGRAM_ERROR_6050 => Self::FeatureNotAvailable,
            PROGRAM_ERROR_6051 => Self::MissingCollectionAccount,
            PROGRAM_ERROR_6052 => Self::AssetDataTooLong,
            PROGRAM_ERROR_6053 => Self::AlreadyInCollection,
            PROGRAM_ERROR_6054 => Self::AlreadyNotInCollection,
            PROGRAM_ERROR_6055 => Self::MissingMplCoreCpiSigner,
            PROGRAM_ERROR_6056 => Self::AssetNotFrozen,
            PROGRAM_ERROR_6057 => Self::CollectionMustHaveRoyaltiesPlugin,
            PROGRAM_ERROR_6058 => Self::InheritedSellerFeeCannotHaveLeafCreators,
            PROGRAM_ERROR_6059 => Self::CannotRemoveFromCollectionWithInheritedSellerFee,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::AssetOwnerMismatch => PROGRAM_ERROR_6000,
            Self::PublicKeyMismatch => PROGRAM_ERROR_6001,
            Self::HashingMismatch => PROGRAM_ERROR_6002,
            Self::UnsupportedSchemaVersion => PROGRAM_ERROR_6003,
            Self::CreatorShareSum => PROGRAM_ERROR_6004,
            Self::DuplicateCreatorAddress => PROGRAM_ERROR_6005,
            Self::CreatorNotVerified => PROGRAM_ERROR_6006,
            Self::CreatorNotFound => PROGRAM_ERROR_6007,
            Self::NoCreators => PROGRAM_ERROR_6008,
            Self::CreatorHashMismatch => PROGRAM_ERROR_6009,
            Self::DataHashMismatch => PROGRAM_ERROR_6010,
            Self::CreatorsTooLong => PROGRAM_ERROR_6011,
            Self::NameTooLong => PROGRAM_ERROR_6012,
            Self::SymbolTooLong => PROGRAM_ERROR_6013,
            Self::UriTooLong => PROGRAM_ERROR_6014,
            Self::BasisPointsExceedMax => PROGRAM_ERROR_6015,
            Self::TreeCreatorOrDelegateMustSign => PROGRAM_ERROR_6016,
            Self::InsufficientMintApprovals => PROGRAM_ERROR_6017,
            Self::NumericalOverflow => PROGRAM_ERROR_6018,
            Self::IncorrectOwner => PROGRAM_ERROR_6019,
            Self::CannotVerifyCollection => PROGRAM_ERROR_6020,
            Self::CollectionNotFound => PROGRAM_ERROR_6021,
            Self::CollectionAlreadyVerified => PROGRAM_ERROR_6022,
            Self::CollectionAlreadyUnverified => PROGRAM_ERROR_6023,
            Self::IncorrectMetadataUpdateAuthority => PROGRAM_ERROR_6024,
            Self::OwnerOrDelegateMismatch => PROGRAM_ERROR_6025,
            Self::CollectionMustBeSized => PROGRAM_ERROR_6026,
            Self::MetadataMintMismatch => PROGRAM_ERROR_6027,
            Self::InvalidCollectionAuthority => PROGRAM_ERROR_6028,
            Self::InvalidDelegateRecordPda => PROGRAM_ERROR_6029,
            Self::EditionMismatch => PROGRAM_ERROR_6030,
            Self::CollectionMustBeMasterEdition => PROGRAM_ERROR_6031,
            Self::ExternalErrorConversion => PROGRAM_ERROR_6032,
            Self::DecompressionDisabled => PROGRAM_ERROR_6033,
            Self::MissingCollectionMint => PROGRAM_ERROR_6034,
            Self::MissingCollectionMetadata => PROGRAM_ERROR_6035,
            Self::CollectionMismatch => PROGRAM_ERROR_6036,
            Self::MetadataNotMutable => PROGRAM_ERROR_6037,
            Self::PrimarySaleCanOnlyBeFlipped => PROGRAM_ERROR_6038,
            Self::CreatorDidNotUnverify => PROGRAM_ERROR_6039,
            Self::OnlyNonFungibleSupported => PROGRAM_ERROR_6040,
            Self::CanopySizeTooSmall => PROGRAM_ERROR_6041,
            Self::InvalidLogWrapper => PROGRAM_ERROR_6042,
            Self::InvalidCompressionProgram => PROGRAM_ERROR_6043,
            Self::LeafMustBeDelegated => PROGRAM_ERROR_6044,
            Self::AssetFrozen => PROGRAM_ERROR_6045,
            Self::AssetNonTransferable => PROGRAM_ERROR_6046,
            Self::InvalidAuthority => PROGRAM_ERROR_6047,
            Self::CollectionFrozen => PROGRAM_ERROR_6048,
            Self::CoreCollectionsMustHavePlugin => PROGRAM_ERROR_6049,
            Self::FeatureNotAvailable => PROGRAM_ERROR_6050,
            Self::MissingCollectionAccount => PROGRAM_ERROR_6051,
            Self::AssetDataTooLong => PROGRAM_ERROR_6052,
            Self::AlreadyInCollection => PROGRAM_ERROR_6053,
            Self::AlreadyNotInCollection => PROGRAM_ERROR_6054,
            Self::MissingMplCoreCpiSigner => PROGRAM_ERROR_6055,
            Self::AssetNotFrozen => PROGRAM_ERROR_6056,
            Self::CollectionMustHaveRoyaltiesPlugin => PROGRAM_ERROR_6057,
            Self::InheritedSellerFeeCannotHaveLeafCreators => PROGRAM_ERROR_6058,
            Self::CannotRemoveFromCollectionWithInheritedSellerFee => PROGRAM_ERROR_6059,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::AssetOwnerMismatch => Some("Asset owner does not match"),
            Self::PublicKeyMismatch => Some("Public key mismatch"),
            Self::HashingMismatch => Some("Hashing mismatch within leaf schema"),
            Self::UnsupportedSchemaVersion => Some("Unsupported schema version"),
            Self::CreatorShareSum => Some("Creator shares must sum to 100"),
            Self::DuplicateCreatorAddress => Some("No duplicate creator addresses in metadata"),
            Self::CreatorNotVerified => Some("Creator did not verify the metadata"),
            Self::CreatorNotFound => Some("Creator not found in creator Vec"),
            Self::NoCreators => Some("No creators in creator Vec"),
            Self::CreatorHashMismatch => {
                Some("User-provided creator Vec must result in same user-provided creator hash")
            }
            Self::DataHashMismatch => {
                Some("User-provided metadata must result in same user-provided data hash")
            }
            Self::CreatorsTooLong => Some("Creators list too long"),
            Self::NameTooLong => Some("Name in metadata is too long"),
            Self::SymbolTooLong => Some("Symbol in metadata is too long"),
            Self::UriTooLong => Some("URI in metadata is too long"),
            Self::BasisPointsExceedMax => Some("Basis points in metadata cannot exceed 10000"),
            Self::TreeCreatorOrDelegateMustSign => Some("Tree creator or tree delegate must sign"),
            Self::InsufficientMintApprovals => Some("Not enough unapproved mints left"),
            Self::NumericalOverflow => Some("Numerical overflow error"),
            Self::IncorrectOwner => Some("Incorrect account owner"),
            Self::CannotVerifyCollection => Some("Cannot verify collection in this instruction"),
            Self::CollectionNotFound => Some("Collection not found on metadata"),
            Self::CollectionAlreadyVerified => Some("Collection item is already verified"),
            Self::CollectionAlreadyUnverified => Some("Collection item is already unverified"),
            Self::IncorrectMetadataUpdateAuthority => {
                Some("Incorrect leaf metadata update authority")
            }
            Self::OwnerOrDelegateMismatch => {
                Some("This transaction must be signed by either the leaf owner or leaf delegate")
            }
            Self::CollectionMustBeSized => Some("Collection must be a sized collection"),
            Self::MetadataMintMismatch => Some("Metadata mint does not match collection mint"),
            Self::InvalidCollectionAuthority => Some("Invalid collection authority"),
            Self::InvalidDelegateRecordPda => Some("Invalid delegate record PDA derivation"),
            Self::EditionMismatch => Some("Edition account doesn't match collection"),
            Self::CollectionMustBeMasterEdition => {
                Some("Collection must be a unique master edition v2")
            }
            Self::ExternalErrorConversion => {
                Some("Could not convert external error to BubblegumError")
            }
            Self::DecompressionDisabled => Some("Decompression is disabled for this tree"),
            Self::MissingCollectionMint => Some("Missing collection mint account"),
            Self::MissingCollectionMetadata => Some("Missing collection metadata account"),
            Self::CollectionMismatch => Some("Collection mismatch"),
            Self::MetadataNotMutable => Some("Metadata not mutable"),
            Self::PrimarySaleCanOnlyBeFlipped => Some("Can only update primary sale to true"),
            Self::CreatorDidNotUnverify => Some("Creator did not unverify the metadata"),
            Self::OnlyNonFungibleSupported => Some("Only NonFungible standard is supported"),
            Self::CanopySizeTooSmall => Some("Canopy size should be set bigger for this tree"),
            Self::InvalidLogWrapper => Some("Invalid log wrapper program"),
            Self::InvalidCompressionProgram => Some("Invalid compression program"),
            Self::LeafMustBeDelegated => {
                Some("Leaf must be delegated to someone other than the leaf owner")
            }
            Self::AssetFrozen => Some("Asset is frozen"),
            Self::AssetNonTransferable => Some("Asset is non-transferable"),
            Self::InvalidAuthority => Some("Invalid authority"),
            Self::CollectionFrozen => Some("Collection is frozen"),
            Self::CoreCollectionsMustHavePlugin => {
                Some("Core collections must have the Bubblegum V2 plugin on them")
            }
            Self::FeatureNotAvailable => Some("Feature not currently available"),
            Self::MissingCollectionAccount => Some("Missing collection account"),
            Self::AssetDataTooLong => Some("Asset data length too long"),
            Self::AlreadyInCollection => Some("Item is already in the collection"),
            Self::AlreadyNotInCollection => Some("Item is already not in a collection"),
            Self::MissingMplCoreCpiSigner => Some("Missing mpl-core CPI signer account"),
            Self::AssetNotFrozen => Some("Asset is not frozen"),
            Self::CollectionMustHaveRoyaltiesPlugin => Some(
                "Core collections must have the Royalties plugin to inherit seller fee basis points",
            ),
            Self::InheritedSellerFeeCannotHaveLeafCreators => {
                Some("Inherited seller fee basis points cannot be used with leaf-level creators")
            }
            Self::CannotRemoveFromCollectionWithInheritedSellerFee => {
                Some("Cannot remove from collection while seller fee basis points are inherited")
            }
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MplBubblegumInstructionData {
    /// Data for tag 116,110,29,56,107,219,42,93.
    Burn(Burn),
    /// Data for tag 115,210,34,240,232,143,183,16.
    BurnV2(BurnV2),
    /// Data for tag 111,76,232,50,39,175,48,242.
    CancelRedeem(CancelRedeem),
    /// Data for tag 45,172,6,94,28,90,157,70.
    CloseTreeV2(CloseTreeV2),
    /// Data for tag 21,11,159,47,4,195,106,56.
    CollectV2(CollectV2),
    /// Data for tag 82,193,176,117,176,21,115,253.
    Compress(Compress),
    /// Data for tag 165,83,136,142,89,202,47,220.
    CreateTree(CreateTree),
    /// Data for tag 55,99,95,215,142,203,227,205.
    CreateTreeV2(CreateTreeV2),
    /// Data for tag 54,85,76,70,228,250,164,81.
    DecompressV1(DecompressV1),
    /// Data for tag 90,147,75,178,85,88,4,137.
    Delegate(Delegate),
    /// Data for tag 17,229,35,218,190,241,250,123.
    DelegateAndFreezeV2(DelegateAndFreezeV2),
    /// Data for tag 95,87,125,140,181,131,128,227.
    DelegateV2(DelegateV2),
    /// Data for tag 200,151,244,102,16,195,255,3.
    FreezeV2(FreezeV2),
    /// Data for tag 153,18,178,47,197,158,86,15.
    MintToCollectionV1(MintToCollectionV1),
    /// Data for tag 145,98,192,118,184,147,118,104.
    MintV1(MintV1),
    /// Data for tag 120,121,23,146,173,110,199,205.
    MintV2(MintV2),
    /// Data for tag 184,12,86,149,70,196,97,225.
    Redeem(Redeem),
    /// Data for tag 235,242,121,216,158,234,180,234.
    SetAndVerifyCollection(SetAndVerifyCollection),
    /// Data for tag 229,35,61,91,15,14,99,160.
    SetCollectionV2(SetCollectionV2),
    /// Data for tag 18,135,238,168,246,195,61,115.
    SetDecompressableState(SetDecompressableState),
    /// Data for tag 82,104,152,6,149,111,100,13.
    SetDecompressibleState(SetDecompressibleState),
    /// Data for tag 181,141,206,58,242,199,152,168.
    SetNonTransferableV2(SetNonTransferableV2),
    /// Data for tag 253,118,66,37,190,49,154,102.
    SetTreeDelegate(SetTreeDelegate),
    /// Data for tag 86,214,190,37,167,4,28,116.
    ThawAndRevokeV2(ThawAndRevokeV2),
    /// Data for tag 96,133,101,93,82,220,146,191.
    ThawV2(ThawV2),
    /// Data for tag 163,52,200,231,140,3,69,186.
    Transfer(Transfer),
    /// Data for tag 119,40,6,235,234,221,248,49.
    TransferV2(TransferV2),
    /// Data for tag 250,251,42,106,41,137,186,168.
    UnverifyCollection(UnverifyCollection),
    /// Data for tag 107,178,57,39,105,115,112,152.
    UnverifyCreator(UnverifyCreator),
    /// Data for tag 174,112,29,142,230,100,239,7.
    UnverifyCreatorV2(UnverifyCreatorV2),
    /// Data for tag 59,56,111,43,95,14,11,61.
    UpdateAssetDataV2(UpdateAssetDataV2),
    /// Data for tag 170,182,43,239,97,78,225,186.
    UpdateMetadata(UpdateMetadata),
    /// Data for tag 43,103,89,42,121,242,62,72.
    UpdateMetadataV2(UpdateMetadataV2),
    /// Data for tag 56,113,101,253,79,55,122,169.
    VerifyCollection(VerifyCollection),
    /// Data for tag 52,17,96,132,71,4,85,194.
    VerifyCreator(VerifyCreator),
    /// Data for tag 85,138,140,42,22,241,118,102.
    VerifyCreatorV2(VerifyCreatorV2),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct MplBubblegumInstructionDataCodec;
impl Encoder<MplBubblegumInstructionData> for MplBubblegumInstructionDataCodec {
    fn encoded_len(&self, value: &MplBubblegumInstructionData) -> Result<usize, Error> {
        Self::encoded_len_0(value)
    }
    fn write(
        &self,
        value: &MplBubblegumInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Self::write_0(value, writer)
    }
}
impl Decoder<MplBubblegumInstructionData> for MplBubblegumInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MplBubblegumInstructionData, Error> {
        let tag = reader.remaining();
        if tag.is_empty() {
            return Err(Error::Truncated);
        }
        Self::read_0(tag, reader)
    }
}
impl MplBubblegumInstructionDataCodec {
    fn encoded_len_0(value: &MplBubblegumInstructionData) -> Result<usize, Error> {
        match value {
            MplBubblegumInstructionData::Burn(value) => BurnCodec.encoded_len(value),
            MplBubblegumInstructionData::BurnV2(value) => BurnV2Codec.encoded_len(value),
            MplBubblegumInstructionData::CancelRedeem(value) => {
                CancelRedeemCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::CloseTreeV2(value) => CloseTreeV2Codec.encoded_len(value),
            MplBubblegumInstructionData::CollectV2(value) => CollectV2Codec.encoded_len(value),
            MplBubblegumInstructionData::Compress(value) => CompressCodec.encoded_len(value),
            MplBubblegumInstructionData::CreateTree(value) => CreateTreeCodec.encoded_len(value),
            MplBubblegumInstructionData::CreateTreeV2(value) => {
                CreateTreeV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::DecompressV1(value) => {
                DecompressV1Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::Delegate(value) => DelegateCodec.encoded_len(value),
            MplBubblegumInstructionData::DelegateAndFreezeV2(value) => {
                DelegateAndFreezeV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::DelegateV2(value) => DelegateV2Codec.encoded_len(value),
            MplBubblegumInstructionData::FreezeV2(value) => FreezeV2Codec.encoded_len(value),
            MplBubblegumInstructionData::MintToCollectionV1(value) => {
                MintToCollectionV1Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::MintV1(value) => MintV1Codec.encoded_len(value),
            MplBubblegumInstructionData::MintV2(value) => MintV2Codec.encoded_len(value),
            _ => Self::encoded_len_1(value),
        }
    }
    fn encoded_len_1(value: &MplBubblegumInstructionData) -> Result<usize, Error> {
        match value {
            MplBubblegumInstructionData::Redeem(value) => RedeemCodec.encoded_len(value),
            MplBubblegumInstructionData::SetAndVerifyCollection(value) => {
                SetAndVerifyCollectionCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::SetCollectionV2(value) => {
                SetCollectionV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::SetDecompressableState(value) => {
                SetDecompressableStateCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::SetDecompressibleState(value) => {
                SetDecompressibleStateCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::SetNonTransferableV2(value) => {
                SetNonTransferableV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::SetTreeDelegate(value) => {
                SetTreeDelegateCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::ThawAndRevokeV2(value) => {
                ThawAndRevokeV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::ThawV2(value) => ThawV2Codec.encoded_len(value),
            MplBubblegumInstructionData::Transfer(value) => TransferCodec.encoded_len(value),
            MplBubblegumInstructionData::TransferV2(value) => TransferV2Codec.encoded_len(value),
            MplBubblegumInstructionData::UnverifyCollection(value) => {
                UnverifyCollectionCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::UnverifyCreator(value) => {
                UnverifyCreatorCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::UnverifyCreatorV2(value) => {
                UnverifyCreatorV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::UpdateAssetDataV2(value) => {
                UpdateAssetDataV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::UpdateMetadata(value) => {
                UpdateMetadataCodec.encoded_len(value)
            }
            _ => Self::encoded_len_2(value),
        }
    }
    fn encoded_len_2(value: &MplBubblegumInstructionData) -> Result<usize, Error> {
        match value {
            MplBubblegumInstructionData::UpdateMetadataV2(value) => {
                UpdateMetadataV2Codec.encoded_len(value)
            }
            MplBubblegumInstructionData::VerifyCollection(value) => {
                VerifyCollectionCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::VerifyCreator(value) => {
                VerifyCreatorCodec.encoded_len(value)
            }
            MplBubblegumInstructionData::VerifyCreatorV2(value) => {
                VerifyCreatorV2Codec.encoded_len(value)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn write_0(value: &MplBubblegumInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MplBubblegumInstructionData::Burn(value) => BurnCodec.write(value, writer),
            MplBubblegumInstructionData::BurnV2(value) => BurnV2Codec.write(value, writer),
            MplBubblegumInstructionData::CancelRedeem(value) => {
                CancelRedeemCodec.write(value, writer)
            }
            MplBubblegumInstructionData::CloseTreeV2(value) => {
                CloseTreeV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::CollectV2(value) => CollectV2Codec.write(value, writer),
            MplBubblegumInstructionData::Compress(value) => CompressCodec.write(value, writer),
            MplBubblegumInstructionData::CreateTree(value) => CreateTreeCodec.write(value, writer),
            MplBubblegumInstructionData::CreateTreeV2(value) => {
                CreateTreeV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::DecompressV1(value) => {
                DecompressV1Codec.write(value, writer)
            }
            MplBubblegumInstructionData::Delegate(value) => DelegateCodec.write(value, writer),
            MplBubblegumInstructionData::DelegateAndFreezeV2(value) => {
                DelegateAndFreezeV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::DelegateV2(value) => DelegateV2Codec.write(value, writer),
            MplBubblegumInstructionData::FreezeV2(value) => FreezeV2Codec.write(value, writer),
            MplBubblegumInstructionData::MintToCollectionV1(value) => {
                MintToCollectionV1Codec.write(value, writer)
            }
            MplBubblegumInstructionData::MintV1(value) => MintV1Codec.write(value, writer),
            MplBubblegumInstructionData::MintV2(value) => MintV2Codec.write(value, writer),
            _ => Self::write_1(value, writer),
        }
    }
    fn write_1(value: &MplBubblegumInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MplBubblegumInstructionData::Redeem(value) => RedeemCodec.write(value, writer),
            MplBubblegumInstructionData::SetAndVerifyCollection(value) => {
                SetAndVerifyCollectionCodec.write(value, writer)
            }
            MplBubblegumInstructionData::SetCollectionV2(value) => {
                SetCollectionV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::SetDecompressableState(value) => {
                SetDecompressableStateCodec.write(value, writer)
            }
            MplBubblegumInstructionData::SetDecompressibleState(value) => {
                SetDecompressibleStateCodec.write(value, writer)
            }
            MplBubblegumInstructionData::SetNonTransferableV2(value) => {
                SetNonTransferableV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::SetTreeDelegate(value) => {
                SetTreeDelegateCodec.write(value, writer)
            }
            MplBubblegumInstructionData::ThawAndRevokeV2(value) => {
                ThawAndRevokeV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::ThawV2(value) => ThawV2Codec.write(value, writer),
            MplBubblegumInstructionData::Transfer(value) => TransferCodec.write(value, writer),
            MplBubblegumInstructionData::TransferV2(value) => TransferV2Codec.write(value, writer),
            MplBubblegumInstructionData::UnverifyCollection(value) => {
                UnverifyCollectionCodec.write(value, writer)
            }
            MplBubblegumInstructionData::UnverifyCreator(value) => {
                UnverifyCreatorCodec.write(value, writer)
            }
            MplBubblegumInstructionData::UnverifyCreatorV2(value) => {
                UnverifyCreatorV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::UpdateAssetDataV2(value) => {
                UpdateAssetDataV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::UpdateMetadata(value) => {
                UpdateMetadataCodec.write(value, writer)
            }
            _ => Self::write_2(value, writer),
        }
    }
    fn write_2(value: &MplBubblegumInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MplBubblegumInstructionData::UpdateMetadataV2(value) => {
                UpdateMetadataV2Codec.write(value, writer)
            }
            MplBubblegumInstructionData::VerifyCollection(value) => {
                VerifyCollectionCodec.write(value, writer)
            }
            MplBubblegumInstructionData::VerifyCreator(value) => {
                VerifyCreatorCodec.write(value, writer)
            }
            MplBubblegumInstructionData::VerifyCreatorV2(value) => {
                VerifyCreatorV2Codec.write(value, writer)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn read_0(tag: &[u8], reader: &mut Reader<'_>) -> Result<MplBubblegumInstructionData, Error> {
        match tag {
            [116, 110, 29, 56, 107, 219, 42, 93, ..] => {
                Ok(MplBubblegumInstructionData::Burn(BurnCodec.read(reader)?))
            }
            [115, 210, 34, 240, 232, 143, 183, 16, ..] => Ok(MplBubblegumInstructionData::BurnV2(
                BurnV2Codec.read(reader)?,
            )),
            [111, 76, 232, 50, 39, 175, 48, 242, ..] => Ok(
                MplBubblegumInstructionData::CancelRedeem(CancelRedeemCodec.read(reader)?),
            ),
            [45, 172, 6, 94, 28, 90, 157, 70, ..] => Ok(MplBubblegumInstructionData::CloseTreeV2(
                CloseTreeV2Codec.read(reader)?,
            )),
            [21, 11, 159, 47, 4, 195, 106, 56, ..] => Ok(MplBubblegumInstructionData::CollectV2(
                CollectV2Codec.read(reader)?,
            )),
            [82, 193, 176, 117, 176, 21, 115, 253, ..] => Ok(
                MplBubblegumInstructionData::Compress(CompressCodec.read(reader)?),
            ),
            [165, 83, 136, 142, 89, 202, 47, 220, ..] => Ok(
                MplBubblegumInstructionData::CreateTree(CreateTreeCodec.read(reader)?),
            ),
            [55, 99, 95, 215, 142, 203, 227, 205, ..] => Ok(
                MplBubblegumInstructionData::CreateTreeV2(CreateTreeV2Codec.read(reader)?),
            ),
            [54, 85, 76, 70, 228, 250, 164, 81, ..] => Ok(
                MplBubblegumInstructionData::DecompressV1(DecompressV1Codec.read(reader)?),
            ),
            [90, 147, 75, 178, 85, 88, 4, 137, ..] => Ok(MplBubblegumInstructionData::Delegate(
                DelegateCodec.read(reader)?,
            )),
            [17, 229, 35, 218, 190, 241, 250, 123, ..] => {
                Ok(MplBubblegumInstructionData::DelegateAndFreezeV2(
                    DelegateAndFreezeV2Codec.read(reader)?,
                ))
            }
            [95, 87, 125, 140, 181, 131, 128, 227, ..] => Ok(
                MplBubblegumInstructionData::DelegateV2(DelegateV2Codec.read(reader)?),
            ),
            [200, 151, 244, 102, 16, 195, 255, 3, ..] => Ok(MplBubblegumInstructionData::FreezeV2(
                FreezeV2Codec.read(reader)?,
            )),
            [153, 18, 178, 47, 197, 158, 86, 15, ..] => {
                Ok(MplBubblegumInstructionData::MintToCollectionV1(
                    MintToCollectionV1Codec.read(reader)?,
                ))
            }
            [145, 98, 192, 118, 184, 147, 118, 104, ..] => Ok(MplBubblegumInstructionData::MintV1(
                MintV1Codec.read(reader)?,
            )),
            [120, 121, 23, 146, 173, 110, 199, 205, ..] => Ok(MplBubblegumInstructionData::MintV2(
                MintV2Codec.read(reader)?,
            )),
            _ => Self::read_1(tag, reader),
        }
    }
    fn read_1(tag: &[u8], reader: &mut Reader<'_>) -> Result<MplBubblegumInstructionData, Error> {
        match tag {
            [184, 12, 86, 149, 70, 196, 97, 225, ..] => Ok(MplBubblegumInstructionData::Redeem(
                RedeemCodec.read(reader)?,
            )),
            [235, 242, 121, 216, 158, 234, 180, 234, ..] => {
                Ok(MplBubblegumInstructionData::SetAndVerifyCollection(
                    SetAndVerifyCollectionCodec.read(reader)?,
                ))
            }
            [229, 35, 61, 91, 15, 14, 99, 160, ..] => Ok(
                MplBubblegumInstructionData::SetCollectionV2(SetCollectionV2Codec.read(reader)?),
            ),
            [18, 135, 238, 168, 246, 195, 61, 115, ..] => {
                Ok(MplBubblegumInstructionData::SetDecompressableState(
                    SetDecompressableStateCodec.read(reader)?,
                ))
            }
            [82, 104, 152, 6, 149, 111, 100, 13, ..] => {
                Ok(MplBubblegumInstructionData::SetDecompressibleState(
                    SetDecompressibleStateCodec.read(reader)?,
                ))
            }
            [181, 141, 206, 58, 242, 199, 152, 168, ..] => {
                Ok(MplBubblegumInstructionData::SetNonTransferableV2(
                    SetNonTransferableV2Codec.read(reader)?,
                ))
            }
            [253, 118, 66, 37, 190, 49, 154, 102, ..] => Ok(
                MplBubblegumInstructionData::SetTreeDelegate(SetTreeDelegateCodec.read(reader)?),
            ),
            [86, 214, 190, 37, 167, 4, 28, 116, ..] => Ok(
                MplBubblegumInstructionData::ThawAndRevokeV2(ThawAndRevokeV2Codec.read(reader)?),
            ),
            [96, 133, 101, 93, 82, 220, 146, 191, ..] => Ok(MplBubblegumInstructionData::ThawV2(
                ThawV2Codec.read(reader)?,
            )),
            [163, 52, 200, 231, 140, 3, 69, 186, ..] => Ok(MplBubblegumInstructionData::Transfer(
                TransferCodec.read(reader)?,
            )),
            [119, 40, 6, 235, 234, 221, 248, 49, ..] => Ok(
                MplBubblegumInstructionData::TransferV2(TransferV2Codec.read(reader)?),
            ),
            [250, 251, 42, 106, 41, 137, 186, 168, ..] => {
                Ok(MplBubblegumInstructionData::UnverifyCollection(
                    UnverifyCollectionCodec.read(reader)?,
                ))
            }
            [107, 178, 57, 39, 105, 115, 112, 152, ..] => Ok(
                MplBubblegumInstructionData::UnverifyCreator(UnverifyCreatorCodec.read(reader)?),
            ),
            [174, 112, 29, 142, 230, 100, 239, 7, ..] => {
                Ok(MplBubblegumInstructionData::UnverifyCreatorV2(
                    UnverifyCreatorV2Codec.read(reader)?,
                ))
            }
            [59, 56, 111, 43, 95, 14, 11, 61, ..] => {
                Ok(MplBubblegumInstructionData::UpdateAssetDataV2(
                    UpdateAssetDataV2Codec.read(reader)?,
                ))
            }
            [170, 182, 43, 239, 97, 78, 225, 186, ..] => Ok(
                MplBubblegumInstructionData::UpdateMetadata(UpdateMetadataCodec.read(reader)?),
            ),
            _ => Self::read_2(tag, reader),
        }
    }
    fn read_2(tag: &[u8], reader: &mut Reader<'_>) -> Result<MplBubblegumInstructionData, Error> {
        match tag {
            [43, 103, 89, 42, 121, 242, 62, 72, ..] => Ok(
                MplBubblegumInstructionData::UpdateMetadataV2(UpdateMetadataV2Codec.read(reader)?),
            ),
            [56, 113, 101, 253, 79, 55, 122, 169, ..] => Ok(
                MplBubblegumInstructionData::VerifyCollection(VerifyCollectionCodec.read(reader)?),
            ),
            [52, 17, 96, 132, 71, 4, 85, 194, ..] => Ok(
                MplBubblegumInstructionData::VerifyCreator(VerifyCreatorCodec.read(reader)?),
            ),
            [85, 138, 140, 42, 22, 241, 118, 102, ..] => Ok(
                MplBubblegumInstructionData::VerifyCreatorV2(VerifyCreatorV2Codec.read(reader)?),
            ),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a literal header without validating its payload.
pub fn identify_instruction(data: &[u8]) -> Result<&[u8], Error> {
    match data {
        [116, 110, 29, 56, 107, 219, 42, 93, ..] => Ok(&data[..8]),
        [115, 210, 34, 240, 232, 143, 183, 16, ..] => Ok(&data[..8]),
        [111, 76, 232, 50, 39, 175, 48, 242, ..] => Ok(&data[..8]),
        [45, 172, 6, 94, 28, 90, 157, 70, ..] => Ok(&data[..8]),
        [21, 11, 159, 47, 4, 195, 106, 56, ..] => Ok(&data[..8]),
        [82, 193, 176, 117, 176, 21, 115, 253, ..] => Ok(&data[..8]),
        [165, 83, 136, 142, 89, 202, 47, 220, ..] => Ok(&data[..8]),
        [55, 99, 95, 215, 142, 203, 227, 205, ..] => Ok(&data[..8]),
        [54, 85, 76, 70, 228, 250, 164, 81, ..] => Ok(&data[..8]),
        [90, 147, 75, 178, 85, 88, 4, 137, ..] => Ok(&data[..8]),
        [17, 229, 35, 218, 190, 241, 250, 123, ..] => Ok(&data[..8]),
        [95, 87, 125, 140, 181, 131, 128, 227, ..] => Ok(&data[..8]),
        [200, 151, 244, 102, 16, 195, 255, 3, ..] => Ok(&data[..8]),
        [153, 18, 178, 47, 197, 158, 86, 15, ..] => Ok(&data[..8]),
        [145, 98, 192, 118, 184, 147, 118, 104, ..] => Ok(&data[..8]),
        [120, 121, 23, 146, 173, 110, 199, 205, ..] => Ok(&data[..8]),
        [184, 12, 86, 149, 70, 196, 97, 225, ..] => Ok(&data[..8]),
        [235, 242, 121, 216, 158, 234, 180, 234, ..] => Ok(&data[..8]),
        [229, 35, 61, 91, 15, 14, 99, 160, ..] => Ok(&data[..8]),
        [18, 135, 238, 168, 246, 195, 61, 115, ..] => Ok(&data[..8]),
        [82, 104, 152, 6, 149, 111, 100, 13, ..] => Ok(&data[..8]),
        [181, 141, 206, 58, 242, 199, 152, 168, ..] => Ok(&data[..8]),
        [253, 118, 66, 37, 190, 49, 154, 102, ..] => Ok(&data[..8]),
        [86, 214, 190, 37, 167, 4, 28, 116, ..] => Ok(&data[..8]),
        [96, 133, 101, 93, 82, 220, 146, 191, ..] => Ok(&data[..8]),
        [163, 52, 200, 231, 140, 3, 69, 186, ..] => Ok(&data[..8]),
        [119, 40, 6, 235, 234, 221, 248, 49, ..] => Ok(&data[..8]),
        [250, 251, 42, 106, 41, 137, 186, 168, ..] => Ok(&data[..8]),
        [107, 178, 57, 39, 105, 115, 112, 152, ..] => Ok(&data[..8]),
        [174, 112, 29, 142, 230, 100, 239, 7, ..] => Ok(&data[..8]),
        [59, 56, 111, 43, 95, 14, 11, 61, ..] => Ok(&data[..8]),
        [170, 182, 43, 239, 97, 78, 225, 186, ..] => Ok(&data[..8]),
        [43, 103, 89, 42, 121, 242, 62, 72, ..] => Ok(&data[..8]),
        [56, 113, 101, 253, 79, 55, 122, 169, ..] => Ok(&data[..8]),
        [52, 17, 96, 132, 71, 4, 85, 194, ..] => Ok(&data[..8]),
        [85, 138, 140, 42, 22, 241, 118, 102, ..] => Ok(&data[..8]),
        [] => Err(Error::Truncated),
        _ => Err(Error::InvalidDiscriminator),
    }
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<MplBubblegumInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<MplBubblegumInstructionData, Error> {
    match identify_instruction(&instruction.data)? {
        [116, 110, 29, 56, 107, 219, 42, 93, ..] => Ok(MplBubblegumInstructionData::Burn(
            Burn::parse_with_program(program, instruction, limits)?,
        )),
        [115, 210, 34, 240, 232, 143, 183, 16, ..] => Ok(MplBubblegumInstructionData::BurnV2(
            BurnV2::parse_with_program(program, instruction, limits)?,
        )),
        [111, 76, 232, 50, 39, 175, 48, 242, ..] => Ok(MplBubblegumInstructionData::CancelRedeem(
            CancelRedeem::parse_with_program(program, instruction, limits)?,
        )),
        [45, 172, 6, 94, 28, 90, 157, 70, ..] => Ok(MplBubblegumInstructionData::CloseTreeV2(
            CloseTreeV2::parse_with_program(program, instruction, limits)?,
        )),
        [21, 11, 159, 47, 4, 195, 106, 56, ..] => Ok(MplBubblegumInstructionData::CollectV2(
            CollectV2::parse_with_program(program, instruction, limits)?,
        )),
        [82, 193, 176, 117, 176, 21, 115, 253, ..] => Ok(MplBubblegumInstructionData::Compress(
            Compress::parse_with_program(program, instruction, limits)?,
        )),
        [165, 83, 136, 142, 89, 202, 47, 220, ..] => Ok(MplBubblegumInstructionData::CreateTree(
            CreateTree::parse_with_program(program, instruction, limits)?,
        )),
        [55, 99, 95, 215, 142, 203, 227, 205, ..] => Ok(MplBubblegumInstructionData::CreateTreeV2(
            CreateTreeV2::parse_with_program(program, instruction, limits)?,
        )),
        [54, 85, 76, 70, 228, 250, 164, 81, ..] => Ok(MplBubblegumInstructionData::DecompressV1(
            DecompressV1::parse_with_program(program, instruction, limits)?,
        )),
        [90, 147, 75, 178, 85, 88, 4, 137, ..] => Ok(MplBubblegumInstructionData::Delegate(
            Delegate::parse_with_program(program, instruction, limits)?,
        )),
        [17, 229, 35, 218, 190, 241, 250, 123, ..] => {
            Ok(MplBubblegumInstructionData::DelegateAndFreezeV2(
                DelegateAndFreezeV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [95, 87, 125, 140, 181, 131, 128, 227, ..] => Ok(MplBubblegumInstructionData::DelegateV2(
            DelegateV2::parse_with_program(program, instruction, limits)?,
        )),
        [200, 151, 244, 102, 16, 195, 255, 3, ..] => Ok(MplBubblegumInstructionData::FreezeV2(
            FreezeV2::parse_with_program(program, instruction, limits)?,
        )),
        [153, 18, 178, 47, 197, 158, 86, 15, ..] => {
            Ok(MplBubblegumInstructionData::MintToCollectionV1(
                MintToCollectionV1::parse_with_program(program, instruction, limits)?,
            ))
        }
        [145, 98, 192, 118, 184, 147, 118, 104, ..] => Ok(MplBubblegumInstructionData::MintV1(
            MintV1::parse_with_program(program, instruction, limits)?,
        )),
        [120, 121, 23, 146, 173, 110, 199, 205, ..] => Ok(MplBubblegumInstructionData::MintV2(
            MintV2::parse_with_program(program, instruction, limits)?,
        )),
        [184, 12, 86, 149, 70, 196, 97, 225, ..] => Ok(MplBubblegumInstructionData::Redeem(
            Redeem::parse_with_program(program, instruction, limits)?,
        )),
        [235, 242, 121, 216, 158, 234, 180, 234, ..] => {
            Ok(MplBubblegumInstructionData::SetAndVerifyCollection(
                SetAndVerifyCollection::parse_with_program(program, instruction, limits)?,
            ))
        }
        [229, 35, 61, 91, 15, 14, 99, 160, ..] => Ok(MplBubblegumInstructionData::SetCollectionV2(
            SetCollectionV2::parse_with_program(program, instruction, limits)?,
        )),
        [18, 135, 238, 168, 246, 195, 61, 115, ..] => {
            Ok(MplBubblegumInstructionData::SetDecompressableState(
                SetDecompressableState::parse_with_program(program, instruction, limits)?,
            ))
        }
        [82, 104, 152, 6, 149, 111, 100, 13, ..] => {
            Ok(MplBubblegumInstructionData::SetDecompressibleState(
                SetDecompressibleState::parse_with_program(program, instruction, limits)?,
            ))
        }
        [181, 141, 206, 58, 242, 199, 152, 168, ..] => {
            Ok(MplBubblegumInstructionData::SetNonTransferableV2(
                SetNonTransferableV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [253, 118, 66, 37, 190, 49, 154, 102, ..] => {
            Ok(MplBubblegumInstructionData::SetTreeDelegate(
                SetTreeDelegate::parse_with_program(program, instruction, limits)?,
            ))
        }
        [86, 214, 190, 37, 167, 4, 28, 116, ..] => {
            Ok(MplBubblegumInstructionData::ThawAndRevokeV2(
                ThawAndRevokeV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [96, 133, 101, 93, 82, 220, 146, 191, ..] => Ok(MplBubblegumInstructionData::ThawV2(
            ThawV2::parse_with_program(program, instruction, limits)?,
        )),
        [163, 52, 200, 231, 140, 3, 69, 186, ..] => Ok(MplBubblegumInstructionData::Transfer(
            Transfer::parse_with_program(program, instruction, limits)?,
        )),
        [119, 40, 6, 235, 234, 221, 248, 49, ..] => Ok(MplBubblegumInstructionData::TransferV2(
            TransferV2::parse_with_program(program, instruction, limits)?,
        )),
        [250, 251, 42, 106, 41, 137, 186, 168, ..] => {
            Ok(MplBubblegumInstructionData::UnverifyCollection(
                UnverifyCollection::parse_with_program(program, instruction, limits)?,
            ))
        }
        [107, 178, 57, 39, 105, 115, 112, 152, ..] => {
            Ok(MplBubblegumInstructionData::UnverifyCreator(
                UnverifyCreator::parse_with_program(program, instruction, limits)?,
            ))
        }
        [174, 112, 29, 142, 230, 100, 239, 7, ..] => {
            Ok(MplBubblegumInstructionData::UnverifyCreatorV2(
                UnverifyCreatorV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [59, 56, 111, 43, 95, 14, 11, 61, ..] => {
            Ok(MplBubblegumInstructionData::UpdateAssetDataV2(
                UpdateAssetDataV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [170, 182, 43, 239, 97, 78, 225, 186, ..] => {
            Ok(MplBubblegumInstructionData::UpdateMetadata(
                UpdateMetadata::parse_with_program(program, instruction, limits)?,
            ))
        }
        [43, 103, 89, 42, 121, 242, 62, 72, ..] => {
            Ok(MplBubblegumInstructionData::UpdateMetadataV2(
                UpdateMetadataV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        [56, 113, 101, 253, 79, 55, 122, 169, ..] => {
            Ok(MplBubblegumInstructionData::VerifyCollection(
                VerifyCollection::parse_with_program(program, instruction, limits)?,
            ))
        }
        [52, 17, 96, 132, 71, 4, 85, 194, ..] => Ok(MplBubblegumInstructionData::VerifyCreator(
            VerifyCreator::parse_with_program(program, instruction, limits)?,
        )),
        [85, 138, 140, 42, 22, 241, 118, 102, ..] => {
            Ok(MplBubblegumInstructionData::VerifyCreatorV2(
                VerifyCreatorV2::parse_with_program(program, instruction, limits)?,
            ))
        }
        _ => Err(Error::InvalidDiscriminator),
    }
}
struct OwnedString<C>(C);
impl<C: Encoder<str>> Encoder<String> for OwnedString<C> {
    fn encoded_len(&self, value: &String) -> Result<usize, Error> {
        self.0.encoded_len(value.as_str())
    }
    fn write(&self, value: &String, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_str(), writer)
    }
}
impl<C: Decoder<String>> Decoder<String> for OwnedString<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<String, Error> {
        self.0.read(reader)
    }
}

struct OwnedVector<C>(C);
impl<T, C: Encoder<[T]>> Encoder<Vec<T>> for OwnedVector<C> {
    fn encoded_len(&self, value: &Vec<T>) -> Result<usize, Error> {
        self.0.encoded_len(value.as_slice())
    }
    fn write(&self, value: &Vec<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value.as_slice(), writer)
    }
}
impl<T, C: Decoder<Vec<T>>> Decoder<Vec<T>> for OwnedVector<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        self.0.read(reader)
    }
}
use alloc::string::String;
use solana_kit_core::AccountMeta;
use solana_kit_core::AccountRole;
use solana_kit_core::codecs::Affix;
use solana_kit_core::codecs::Bool;
use solana_kit_core::codecs::Counted;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::Utf8;
use solana_kit_core::codecs::{ByteSizePrefix, FrameConsumption};
impl core::hash::Hash for Creator {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.verified, state);
        core::hash::Hash::hash(&self.share, state);
    }
}
impl core::fmt::Display for Creator {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Collection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.verified, state);
        core::hash::Hash::hash(&self.key, state);
    }
}
impl core::fmt::Display for Collection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Uses {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.use_method, state);
        core::hash::Hash::hash(&self.remaining, state);
        core::hash::Hash::hash(&self.total, state);
    }
}
impl core::fmt::Display for Uses {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MetadataArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seller_fee_basis_points, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.edition_nonce, state);
        core::hash::Hash::hash(&self.token_standard, state);
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.uses, state);
        core::hash::Hash::hash(&self.token_program_version, state);
        core::hash::Hash::hash(&self.creators, state);
    }
}
impl core::fmt::Display for MetadataArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MetadataArgsV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seller_fee_basis_points, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.token_standard, state);
        core::hash::Hash::hash(&self.creators, state);
        core::hash::Hash::hash(&self.collection, state);
    }
}
impl core::fmt::Display for MetadataArgsV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.creators, state);
        core::hash::Hash::hash(&self.seller_fee_basis_points, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
    }
}
impl core::fmt::Display for UpdateArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LeafSchemaEvent {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.event_type, state);
        core::hash::Hash::hash(&self.version, state);
        core::hash::Hash::hash(&self.schema, state);
        core::hash::Hash::hash(&self.leaf_hash, state);
    }
}
impl core::fmt::Display for LeafSchemaEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Version {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1 => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::V2 => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for Version {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenProgramVersion {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Original => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Token2022 => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for TokenProgramVersion {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenStandard {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::NonFungible => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::FungibleAsset => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Fungible => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::NonFungibleEdition => {
                core::hash::Hash::hash(&3usize, state);
            }
        }
    }
}
impl core::fmt::Display for TokenStandard {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UseMethod {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Burn => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Multiple => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Single => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for UseMethod {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DecompressibleState {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Enabled => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Disabled => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for DecompressibleState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AssetDataSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Binary => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Json => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::MsgPack => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for AssetDataSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BubblegumEventType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Uninitialized => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::LeafSchemaEvent => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for BubblegumEventType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for InstructionName {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Unknown => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::MintV1 => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Redeem => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::CancelRedeem => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::Transfer => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::Delegate => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::DecompressV1 => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::Compress => {
                core::hash::Hash::hash(&7usize, state);
            }
            Self::Burn => {
                core::hash::Hash::hash(&8usize, state);
            }
            Self::CreateTree => {
                core::hash::Hash::hash(&9usize, state);
            }
            Self::VerifyCreator => {
                core::hash::Hash::hash(&10usize, state);
            }
            Self::UnverifyCreator => {
                core::hash::Hash::hash(&11usize, state);
            }
            Self::VerifyCollection => {
                core::hash::Hash::hash(&12usize, state);
            }
            Self::UnverifyCollection => {
                core::hash::Hash::hash(&13usize, state);
            }
            Self::SetAndVerifyCollection => {
                core::hash::Hash::hash(&14usize, state);
            }
            Self::MintToCollectionV1 => {
                core::hash::Hash::hash(&15usize, state);
            }
            Self::SetDecompressibleState => {
                core::hash::Hash::hash(&16usize, state);
            }
            Self::UpdateMetadata => {
                core::hash::Hash::hash(&17usize, state);
            }
            Self::BurnV2 => {
                core::hash::Hash::hash(&18usize, state);
            }
            Self::CollectV2 => {
                core::hash::Hash::hash(&19usize, state);
            }
            Self::CreateTreeV2 => {
                core::hash::Hash::hash(&20usize, state);
            }
            Self::DelegateAndFreezeV2 => {
                core::hash::Hash::hash(&21usize, state);
            }
            Self::DelegateV2 => {
                core::hash::Hash::hash(&22usize, state);
            }
            Self::FreezeV2 => {
                core::hash::Hash::hash(&23usize, state);
            }
            Self::MintV2 => {
                core::hash::Hash::hash(&24usize, state);
            }
            Self::SetCollectionV2 => {
                core::hash::Hash::hash(&25usize, state);
            }
            Self::SetNonTransferableV2 => {
                core::hash::Hash::hash(&26usize, state);
            }
            Self::ThawAndRevokeV2 => {
                core::hash::Hash::hash(&27usize, state);
            }
            Self::ThawV2 => {
                core::hash::Hash::hash(&28usize, state);
            }
            Self::TransferV2 => {
                core::hash::Hash::hash(&29usize, state);
            }
            Self::UnverifyCreatorV2 => {
                core::hash::Hash::hash(&30usize, state);
            }
            Self::UpdateAssetDataV2 => {
                core::hash::Hash::hash(&31usize, state);
            }
            Self::UpdateMetadataV2 => {
                core::hash::Hash::hash(&32usize, state);
            }
            Self::VerifyCreatorV2 => {
                core::hash::Hash::hash(&33usize, state);
            }
            Self::CloseTreeV2 => {
                core::hash::Hash::hash(&34usize, state);
            }
        }
    }
}
impl core::fmt::Display for InstructionName {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LeafSchemaV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.id, state);
        core::hash::Hash::hash(&self.owner, state);
        core::hash::Hash::hash(&self.delegate, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
    }
}
impl core::fmt::Display for LeafSchemaV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LeafSchemaV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.id, state);
        core::hash::Hash::hash(&self.owner, state);
        core::hash::Hash::hash(&self.delegate, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.collection_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
    }
}
impl core::fmt::Display for LeafSchemaV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LeafSchema {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::V2(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for LeafSchema {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TreeConfig {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.tree_creator, state);
        core::hash::Hash::hash(&self.tree_delegate, state);
        core::hash::Hash::hash(&self.total_mint_capacity, state);
        core::hash::Hash::hash(&self.num_minted, state);
        core::hash::Hash::hash(&self.is_public, state);
        core::hash::Hash::hash(&self.is_decompressible, state);
        core::hash::Hash::hash(&self.version, state);
    }
}
impl core::fmt::Display for TreeConfig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Voucher {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.leaf_schema, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.merkle_tree, state);
    }
}
impl core::fmt::Display for Voucher {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Burn {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for Burn {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for BurnV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CancelRedeem {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
    }
}
impl core::fmt::Display for CancelRedeem {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseTreeV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseTreeV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CollectV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Compress {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for Compress {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateTree {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_depth, state);
        core::hash::Hash::hash(&self.max_buffer_size, state);
        core::hash::Hash::hash(&self.public, state);
    }
}
impl core::fmt::Display for CreateTree {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateTreeV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_depth, state);
        core::hash::Hash::hash(&self.max_buffer_size, state);
        core::hash::Hash::hash(&self.public, state);
    }
}
impl core::fmt::Display for CreateTreeV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DecompressV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.metadata_args, state);
    }
}
impl core::fmt::Display for DecompressV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Delegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for Delegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateAndFreezeV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.collection_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for DelegateAndFreezeV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.collection_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for DelegateV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for FreezeV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for FreezeV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintToCollectionV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.metadata_args, state);
    }
}
impl core::fmt::Display for MintToCollectionV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for MintV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.metadata_args, state);
        core::hash::Hash::hash(&self.asset_data, state);
        core::hash::Hash::hash(&self.asset_data_schema, state);
    }
}
impl core::fmt::Display for MintV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Redeem {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for Redeem {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetAndVerifyCollection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
        core::hash::Hash::hash(&self.collection, state);
    }
}
impl core::fmt::Display for SetAndVerifyCollection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetCollectionV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for SetCollectionV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetDecompressableState {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.decompressable_state, state);
    }
}
impl core::fmt::Display for SetDecompressableState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetDecompressibleState {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.decompressable_state, state);
    }
}
impl core::fmt::Display for SetDecompressibleState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetNonTransferableV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for SetNonTransferableV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetTreeDelegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for SetTreeDelegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ThawAndRevokeV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.collection_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for ThawAndRevokeV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ThawV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for ThawV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Transfer {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for Transfer {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
    }
}
impl core::fmt::Display for TransferV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnverifyCollection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for UnverifyCollection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnverifyCreator {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for UnverifyCreator {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnverifyCreatorV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for UnverifyCreatorV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateAssetDataV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.previous_asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.new_asset_data, state);
        core::hash::Hash::hash(&self.new_asset_data_schema, state);
    }
}
impl core::fmt::Display for UpdateAssetDataV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateMetadata {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.current_metadata, state);
        core::hash::Hash::hash(&self.update_args, state);
    }
}
impl core::fmt::Display for UpdateMetadata {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateMetadataV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.current_metadata, state);
        core::hash::Hash::hash(&self.update_args, state);
    }
}
impl core::fmt::Display for UpdateMetadataV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifyCollection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for VerifyCollection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifyCreator {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.data_hash, state);
        core::hash::Hash::hash(&self.creator_hash, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for VerifyCreator {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifyCreatorV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.root, state);
        core::hash::Hash::hash(&self.asset_data_hash, state);
        core::hash::Hash::hash(&self.flags, state);
        core::hash::Hash::hash(&self.nonce, state);
        core::hash::Hash::hash(&self.index, state);
        core::hash::Hash::hash(&self.message, state);
    }
}
impl core::fmt::Display for VerifyCreatorV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MplBubblegumInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Burn(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BurnV2(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CancelRedeem(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseTreeV2(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CollectV2(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Compress(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateTree(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateTreeV2(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DecompressV1(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Delegate(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DelegateAndFreezeV2(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DelegateV2(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::FreezeV2(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MintToCollectionV1(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MintV1(value) => {
                core::hash::Hash::hash(&14usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MintV2(value) => {
                core::hash::Hash::hash(&15usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Redeem(value) => {
                core::hash::Hash::hash(&16usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetAndVerifyCollection(value) => {
                core::hash::Hash::hash(&17usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetCollectionV2(value) => {
                core::hash::Hash::hash(&18usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetDecompressableState(value) => {
                core::hash::Hash::hash(&19usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetDecompressibleState(value) => {
                core::hash::Hash::hash(&20usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetNonTransferableV2(value) => {
                core::hash::Hash::hash(&21usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetTreeDelegate(value) => {
                core::hash::Hash::hash(&22usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ThawAndRevokeV2(value) => {
                core::hash::Hash::hash(&23usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ThawV2(value) => {
                core::hash::Hash::hash(&24usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Transfer(value) => {
                core::hash::Hash::hash(&25usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferV2(value) => {
                core::hash::Hash::hash(&26usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UnverifyCollection(value) => {
                core::hash::Hash::hash(&27usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UnverifyCreator(value) => {
                core::hash::Hash::hash(&28usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UnverifyCreatorV2(value) => {
                core::hash::Hash::hash(&29usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateAssetDataV2(value) => {
                core::hash::Hash::hash(&30usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateMetadata(value) => {
                core::hash::Hash::hash(&31usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateMetadataV2(value) => {
                core::hash::Hash::hash(&32usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifyCollection(value) => {
                core::hash::Hash::hash(&33usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifyCreator(value) => {
                core::hash::Hash::hash(&34usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifyCreatorV2(value) => {
                core::hash::Hash::hash(&35usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for MplBubblegumInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MplBubblegumError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for MplBubblegumError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for MplBubblegumError {}
