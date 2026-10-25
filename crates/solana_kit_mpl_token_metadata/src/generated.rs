// Generated from a validated Codama model. See README.md and upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Constant, Decoder, Encoder, Error, Limits, Number, Reader, Writer},
};
/// Selected trusted program identity.
pub const PROGRAM: Address = Address::from_bytes([
    11, 112, 101, 177, 227, 209, 124, 69, 56, 157, 82, 127, 107, 4, 195, 205, 88, 184, 108, 115,
    26, 160, 253, 181, 73, 182, 209, 188, 3, 248, 41, 70,
]);
const INSTRUCTION_TAG_0: u8 = 0;
const INSTRUCTION_TAG_1: u8 = 1;
const INSTRUCTION_TAG_2: u8 = 2;
const INSTRUCTION_TAG_3: u8 = 3;
const INSTRUCTION_TAG_4: u8 = 4;
const INSTRUCTION_TAG_5: u8 = 5;
const INSTRUCTION_TAG_6: u8 = 6;
const INSTRUCTION_TAG_7: u8 = 7;
const INSTRUCTION_TAG_8: u8 = 8;
const INSTRUCTION_TAG_9: u8 = 9;
const INSTRUCTION_TAG_10: u8 = 10;
const INSTRUCTION_TAG_11: u8 = 11;
const INSTRUCTION_TAG_12: u8 = 12;
const INSTRUCTION_TAG_13: u8 = 13;
const INSTRUCTION_TAG_14: u8 = 14;
const INSTRUCTION_TAG_15: u8 = 15;
const INSTRUCTION_TAG_16: u8 = 16;
const INSTRUCTION_TAG_17: u8 = 17;
const INSTRUCTION_TAG_18: u8 = 18;
const INSTRUCTION_TAG_19: u8 = 19;
const INSTRUCTION_TAG_20: u8 = 20;
const INSTRUCTION_TAG_21: u8 = 21;
const INSTRUCTION_TAG_22: u8 = 22;
const INSTRUCTION_TAG_23: u8 = 23;
const INSTRUCTION_TAG_24: u8 = 24;
const INSTRUCTION_TAG_25: u8 = 25;
const INSTRUCTION_TAG_26: u8 = 26;
const INSTRUCTION_TAG_27: u8 = 27;
const INSTRUCTION_TAG_28: u8 = 28;
const INSTRUCTION_TAG_29: u8 = 29;
const INSTRUCTION_TAG_30: u8 = 30;
const INSTRUCTION_TAG_31: u8 = 31;
const INSTRUCTION_TAG_32: u8 = 32;
const INSTRUCTION_TAG_33: u8 = 33;
const INSTRUCTION_TAG_34: u8 = 34;
const INSTRUCTION_TAG_35: u8 = 35;
const INSTRUCTION_TAG_36: u8 = 36;
const INSTRUCTION_TAG_37: u8 = 37;
const INSTRUCTION_TAG_38: u8 = 38;
const INSTRUCTION_TAG_39: u8 = 39;
const INSTRUCTION_TAG_40: u8 = 40;
const INSTRUCTION_TAG_41: u8 = 41;
const INSTRUCTION_TAG_42: u8 = 42;
const INSTRUCTION_TAG_43: u8 = 43;
const INSTRUCTION_TAG_44: u8 = 44;
const INSTRUCTION_TAG_45: u8 = 45;
const INSTRUCTION_TAG_46: u8 = 46;
const INSTRUCTION_TAG_47: u8 = 47;
const INSTRUCTION_TAG_48: u8 = 48;
const INSTRUCTION_TAG_49: u8 = 49;
const INSTRUCTION_TAG_50: u8 = 50;
const INSTRUCTION_TAG_51: u8 = 51;
const INSTRUCTION_TAG_52: u8 = 52;
const INSTRUCTION_TAG_53: u8 = 53;
const INSTRUCTION_TAG_54: u8 = 54;
const INSTRUCTION_TAG_55: u8 = 55;
const INSTRUCTION_TAG_56: u8 = 56;
const INSTRUCTION_TAG_57: u8 = 57;
const PROGRAM_ERROR_0: u32 = 0;
const PROGRAM_ERROR_1: u32 = 1;
const PROGRAM_ERROR_2: u32 = 2;
const PROGRAM_ERROR_3: u32 = 3;
const PROGRAM_ERROR_4: u32 = 4;
const PROGRAM_ERROR_5: u32 = 5;
const PROGRAM_ERROR_6: u32 = 6;
const PROGRAM_ERROR_7: u32 = 7;
const PROGRAM_ERROR_8: u32 = 8;
const PROGRAM_ERROR_9: u32 = 9;
const PROGRAM_ERROR_10: u32 = 10;
const PROGRAM_ERROR_11: u32 = 11;
const PROGRAM_ERROR_12: u32 = 12;
const PROGRAM_ERROR_13: u32 = 13;
const PROGRAM_ERROR_14: u32 = 14;
const PROGRAM_ERROR_15: u32 = 15;
const PROGRAM_ERROR_16: u32 = 16;
const PROGRAM_ERROR_17: u32 = 17;
const PROGRAM_ERROR_18: u32 = 18;
const PROGRAM_ERROR_19: u32 = 19;
const PROGRAM_ERROR_20: u32 = 20;
const PROGRAM_ERROR_21: u32 = 21;
const PROGRAM_ERROR_22: u32 = 22;
const PROGRAM_ERROR_23: u32 = 23;
const PROGRAM_ERROR_24: u32 = 24;
const PROGRAM_ERROR_25: u32 = 25;
const PROGRAM_ERROR_26: u32 = 26;
const PROGRAM_ERROR_27: u32 = 27;
const PROGRAM_ERROR_28: u32 = 28;
const PROGRAM_ERROR_29: u32 = 29;
const PROGRAM_ERROR_30: u32 = 30;
const PROGRAM_ERROR_31: u32 = 31;
const PROGRAM_ERROR_32: u32 = 32;
const PROGRAM_ERROR_33: u32 = 33;
const PROGRAM_ERROR_34: u32 = 34;
const PROGRAM_ERROR_35: u32 = 35;
const PROGRAM_ERROR_36: u32 = 36;
const PROGRAM_ERROR_37: u32 = 37;
const PROGRAM_ERROR_38: u32 = 38;
const PROGRAM_ERROR_39: u32 = 39;
const PROGRAM_ERROR_40: u32 = 40;
const PROGRAM_ERROR_41: u32 = 41;
const PROGRAM_ERROR_42: u32 = 42;
const PROGRAM_ERROR_43: u32 = 43;
const PROGRAM_ERROR_44: u32 = 44;
const PROGRAM_ERROR_45: u32 = 45;
const PROGRAM_ERROR_46: u32 = 46;
const PROGRAM_ERROR_47: u32 = 47;
const PROGRAM_ERROR_48: u32 = 48;
const PROGRAM_ERROR_49: u32 = 49;
const PROGRAM_ERROR_50: u32 = 50;
const PROGRAM_ERROR_51: u32 = 51;
const PROGRAM_ERROR_52: u32 = 52;
const PROGRAM_ERROR_53: u32 = 53;
const PROGRAM_ERROR_54: u32 = 54;
const PROGRAM_ERROR_55: u32 = 55;
const PROGRAM_ERROR_56: u32 = 56;
const PROGRAM_ERROR_57: u32 = 57;
const PROGRAM_ERROR_58: u32 = 58;
const PROGRAM_ERROR_59: u32 = 59;
const PROGRAM_ERROR_60: u32 = 60;
const PROGRAM_ERROR_61: u32 = 61;
const PROGRAM_ERROR_62: u32 = 62;
const PROGRAM_ERROR_63: u32 = 63;
const PROGRAM_ERROR_64: u32 = 64;
const PROGRAM_ERROR_65: u32 = 65;
const PROGRAM_ERROR_66: u32 = 66;
const PROGRAM_ERROR_67: u32 = 67;
const PROGRAM_ERROR_68: u32 = 68;
const PROGRAM_ERROR_69: u32 = 69;
const PROGRAM_ERROR_70: u32 = 70;
const PROGRAM_ERROR_71: u32 = 71;
const PROGRAM_ERROR_72: u32 = 72;
const PROGRAM_ERROR_73: u32 = 73;
const PROGRAM_ERROR_74: u32 = 74;
const PROGRAM_ERROR_75: u32 = 75;
const PROGRAM_ERROR_76: u32 = 76;
const PROGRAM_ERROR_77: u32 = 77;
const PROGRAM_ERROR_78: u32 = 78;
const PROGRAM_ERROR_79: u32 = 79;
const PROGRAM_ERROR_80: u32 = 80;
const PROGRAM_ERROR_81: u32 = 81;
const PROGRAM_ERROR_82: u32 = 82;
const PROGRAM_ERROR_83: u32 = 83;
const PROGRAM_ERROR_84: u32 = 84;
const PROGRAM_ERROR_85: u32 = 85;
const PROGRAM_ERROR_86: u32 = 86;
const PROGRAM_ERROR_87: u32 = 87;
const PROGRAM_ERROR_88: u32 = 88;
const PROGRAM_ERROR_89: u32 = 89;
const PROGRAM_ERROR_90: u32 = 90;
const PROGRAM_ERROR_91: u32 = 91;
const PROGRAM_ERROR_92: u32 = 92;
const PROGRAM_ERROR_93: u32 = 93;
const PROGRAM_ERROR_94: u32 = 94;
const PROGRAM_ERROR_95: u32 = 95;
const PROGRAM_ERROR_96: u32 = 96;
const PROGRAM_ERROR_97: u32 = 97;
const PROGRAM_ERROR_98: u32 = 98;
const PROGRAM_ERROR_99: u32 = 99;
const PROGRAM_ERROR_100: u32 = 100;
const PROGRAM_ERROR_101: u32 = 101;
const PROGRAM_ERROR_102: u32 = 102;
const PROGRAM_ERROR_103: u32 = 103;
const PROGRAM_ERROR_104: u32 = 104;
const PROGRAM_ERROR_105: u32 = 105;
const PROGRAM_ERROR_106: u32 = 106;
const PROGRAM_ERROR_107: u32 = 107;
const PROGRAM_ERROR_108: u32 = 108;
const PROGRAM_ERROR_109: u32 = 109;
const PROGRAM_ERROR_110: u32 = 110;
const PROGRAM_ERROR_111: u32 = 111;
const PROGRAM_ERROR_112: u32 = 112;
const PROGRAM_ERROR_113: u32 = 113;
const PROGRAM_ERROR_114: u32 = 114;
const PROGRAM_ERROR_115: u32 = 115;
const PROGRAM_ERROR_116: u32 = 116;
const PROGRAM_ERROR_117: u32 = 117;
const PROGRAM_ERROR_118: u32 = 118;
const PROGRAM_ERROR_119: u32 = 119;
const PROGRAM_ERROR_120: u32 = 120;
const PROGRAM_ERROR_121: u32 = 121;
const PROGRAM_ERROR_122: u32 = 122;
const PROGRAM_ERROR_123: u32 = 123;
const PROGRAM_ERROR_124: u32 = 124;
const PROGRAM_ERROR_125: u32 = 125;
const PROGRAM_ERROR_126: u32 = 126;
const PROGRAM_ERROR_127: u32 = 127;
const PROGRAM_ERROR_128: u32 = 128;
const PROGRAM_ERROR_129: u32 = 129;
const PROGRAM_ERROR_130: u32 = 130;
const PROGRAM_ERROR_131: u32 = 131;
const PROGRAM_ERROR_132: u32 = 132;
const PROGRAM_ERROR_133: u32 = 133;
const PROGRAM_ERROR_134: u32 = 134;
const PROGRAM_ERROR_135: u32 = 135;
const PROGRAM_ERROR_136: u32 = 136;
const PROGRAM_ERROR_137: u32 = 137;
const PROGRAM_ERROR_138: u32 = 138;
const PROGRAM_ERROR_139: u32 = 139;
const PROGRAM_ERROR_140: u32 = 140;
const PROGRAM_ERROR_141: u32 = 141;
const PROGRAM_ERROR_142: u32 = 142;
const PROGRAM_ERROR_143: u32 = 143;
const PROGRAM_ERROR_144: u32 = 144;
const PROGRAM_ERROR_145: u32 = 145;
const PROGRAM_ERROR_146: u32 = 146;
const PROGRAM_ERROR_147: u32 = 147;
const PROGRAM_ERROR_148: u32 = 148;
const PROGRAM_ERROR_149: u32 = 149;
const PROGRAM_ERROR_150: u32 = 150;
const PROGRAM_ERROR_151: u32 = 151;
const PROGRAM_ERROR_152: u32 = 152;
const PROGRAM_ERROR_153: u32 = 153;
const PROGRAM_ERROR_154: u32 = 154;
const PROGRAM_ERROR_155: u32 = 155;
const PROGRAM_ERROR_156: u32 = 156;
const PROGRAM_ERROR_157: u32 = 157;
const PROGRAM_ERROR_158: u32 = 158;
const PROGRAM_ERROR_159: u32 = 159;
const PROGRAM_ERROR_160: u32 = 160;
const PROGRAM_ERROR_161: u32 = 161;
const PROGRAM_ERROR_162: u32 = 162;
const PROGRAM_ERROR_163: u32 = 163;
const PROGRAM_ERROR_164: u32 = 164;
const PROGRAM_ERROR_165: u32 = 165;
const PROGRAM_ERROR_166: u32 = 166;
const PROGRAM_ERROR_167: u32 = 167;
const PROGRAM_ERROR_168: u32 = 168;
const PROGRAM_ERROR_169: u32 = 169;
const PROGRAM_ERROR_170: u32 = 170;
const PROGRAM_ERROR_171: u32 = 171;
const PROGRAM_ERROR_172: u32 = 172;
const PROGRAM_ERROR_173: u32 = 173;
const PROGRAM_ERROR_174: u32 = 174;
const PROGRAM_ERROR_175: u32 = 175;
const PROGRAM_ERROR_176: u32 = 176;
const PROGRAM_ERROR_177: u32 = 177;
const PROGRAM_ERROR_178: u32 = 178;
const PROGRAM_ERROR_179: u32 = 179;
const PROGRAM_ERROR_180: u32 = 180;
const PROGRAM_ERROR_181: u32 = 181;
const PROGRAM_ERROR_182: u32 = 182;
const PROGRAM_ERROR_183: u32 = 183;
const PROGRAM_ERROR_184: u32 = 184;
const PROGRAM_ERROR_185: u32 = 185;
const PROGRAM_ERROR_186: u32 = 186;
const PROGRAM_ERROR_187: u32 = 187;
const PROGRAM_ERROR_188: u32 = 188;
const PROGRAM_ERROR_189: u32 = 189;
const PROGRAM_ERROR_190: u32 = 190;
const PROGRAM_ERROR_191: u32 = 191;
const PROGRAM_ERROR_192: u32 = 192;
const PROGRAM_ERROR_193: u32 = 193;
const PROGRAM_ERROR_194: u32 = 194;
const PROGRAM_ERROR_195: u32 = 195;
const PROGRAM_ERROR_196: u32 = 196;
const PROGRAM_ERROR_197: u32 = 197;
const PROGRAM_ERROR_198: u32 = 198;
const PROGRAM_ERROR_199: u32 = 199;
const PROGRAM_ERROR_200: u32 = 200;
const PROGRAM_ERROR_201: u32 = 201;
const PROGRAM_ERROR_202: u32 = 202;

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
/// Validated data for `Data`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seller_fee_basis_points` wire field.
    pub seller_fee_basis_points: u16,
    /// The `creators` wire field.
    pub creators: Option<Vec<Creator>>,
}
/// Bounded codec for [`Data`].
#[derive(Clone, Copy, Debug)]
pub struct DataCodec;
impl Encoder<Data> for DataCodec {
    fn encoded_len(&self, value: &Data) -> Result<usize, Error> {
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
            .checked_add(
                OptionCodec::u8(OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: CreatorCodec,
                    minimum: 34,
                }))
                .encoded_len(&value.creators)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Data, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 34,
        }))
        .write(&value.creators, writer)?;
        Ok(())
    }
}
impl Decoder<Data> for DataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Data, Error> {
        Ok(Data {
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
            creators: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 34,
            }))
            .read(reader)?,
        })
    }
}
/// Validated data for `DataV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataV2 {
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seller_fee_basis_points` wire field.
    pub seller_fee_basis_points: u16,
    /// The `creators` wire field.
    pub creators: Option<Vec<Creator>>,
    /// The `collection` wire field.
    pub collection: Option<Collection>,
    /// The `uses` wire field.
    pub uses: Option<Uses>,
}
/// Bounded codec for [`DataV2`].
#[derive(Clone, Copy, Debug)]
pub struct DataV2Codec;
impl Encoder<DataV2> for DataV2Codec {
    fn encoded_len(&self, value: &DataV2) -> Result<usize, Error> {
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
            .checked_add(OptionCodec::u8(CollectionCodec).encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(UsesCodec).encoded_len(&value.uses)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &DataV2, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 34,
        }))
        .write(&value.creators, writer)?;
        OptionCodec::u8(CollectionCodec).write(&value.collection, writer)?;
        OptionCodec::u8(UsesCodec).write(&value.uses, writer)?;
        Ok(())
    }
}
impl Decoder<DataV2> for DataV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DataV2, Error> {
        Ok(DataV2 {
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
            creators: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 34,
            }))
            .read(reader)?,
            collection: OptionCodec::u8(CollectionCodec).read(reader)?,
            uses: OptionCodec::u8(UsesCodec).read(reader)?,
        })
    }
}
/// Validated data for `AssetData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetData {
    /// The `name` wire field.
    pub name: String,
    /// The `symbol` wire field.
    pub symbol: String,
    /// The `uri` wire field.
    pub uri: String,
    /// The `seller_fee_basis_points` wire field.
    pub seller_fee_basis_points: u16,
    /// The `creators` wire field.
    pub creators: Option<Vec<Creator>>,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: bool,
    /// The `is_mutable` wire field.
    pub is_mutable: bool,
    /// The `token_standard` wire field.
    pub token_standard: TokenStandard,
    /// The `collection` wire field.
    pub collection: Option<Collection>,
    /// The `uses` wire field.
    pub uses: Option<Uses>,
    /// The `collection_details` wire field.
    pub collection_details: Option<CollectionDetails>,
    /// The `rule_set` wire field.
    pub rule_set: Option<Address>,
}
/// Bounded codec for [`AssetData`].
#[derive(Clone, Copy, Debug)]
pub struct AssetDataCodec;
impl Encoder<AssetData> for AssetDataCodec {
    fn encoded_len(&self, value: &AssetData) -> Result<usize, Error> {
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
            .checked_add(Bool.encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(TokenStandardCodec.encoded_len(&value.token_standard)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(CollectionCodec).encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(UsesCodec).encoded_len(&value.uses)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(CollectionDetailsCodec).encoded_len(&value.collection_details)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.rule_set)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AssetData, writer: &mut Writer<'_>) -> Result<(), Error> {
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
        OptionCodec::u8(OwnedVector(CheckedCounted {
            prefix: Number::<u32>::LE,
            item: CreatorCodec,
            minimum: 34,
        }))
        .write(&value.creators, writer)?;
        Bool.write(&value.primary_sale_happened, writer)?;
        Bool.write(&value.is_mutable, writer)?;
        TokenStandardCodec.write(&value.token_standard, writer)?;
        OptionCodec::u8(CollectionCodec).write(&value.collection, writer)?;
        OptionCodec::u8(UsesCodec).write(&value.uses, writer)?;
        OptionCodec::u8(CollectionDetailsCodec).write(&value.collection_details, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.rule_set, writer)?;
        Ok(())
    }
}
impl Decoder<AssetData> for AssetDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AssetData, Error> {
        Ok(AssetData {
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
            creators: OptionCodec::u8(OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: CreatorCodec,
                minimum: 34,
            }))
            .read(reader)?,
            primary_sale_happened: Bool.read(reader)?,
            is_mutable: Bool.read(reader)?,
            token_standard: TokenStandardCodec.read(reader)?,
            collection: OptionCodec::u8(CollectionCodec).read(reader)?,
            uses: OptionCodec::u8(UsesCodec).read(reader)?,
            collection_details: OptionCodec::u8(CollectionDetailsCodec).read(reader)?,
            rule_set: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
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
/// Validated data for `AuthorizationData`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationData {
    /// The `payload` wire field.
    pub payload: Payload,
}
/// Bounded codec for [`AuthorizationData`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorizationDataCodec;
impl Encoder<AuthorizationData> for AuthorizationDataCodec {
    fn encoded_len(&self, value: &AuthorizationData) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(PayloadCodec.encoded_len(&value.payload)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AuthorizationData, writer: &mut Writer<'_>) -> Result<(), Error> {
        PayloadCodec.write(&value.payload, writer)?;
        Ok(())
    }
}
impl Decoder<AuthorizationData> for AuthorizationDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorizationData, Error> {
        Ok(AuthorizationData {
            payload: PayloadCodec.read(reader)?,
        })
    }
}
/// Validated data for `Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payload {
    /// The `map` wire field.
    pub map: Vec<(String, PayloadType)>,
}
/// Bounded codec for [`Payload`].
#[derive(Clone, Copy, Debug)]
pub struct PayloadCodec;
impl Encoder<Payload> for PayloadCodec {
    fn encoded_len(&self, value: &Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Tuple(
                        OwnedString(ByteSizePrefix {
                            item: Utf8,
                            prefix: Mapped::new(
                                Number::<u32>::LE,
                                |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                                |value: u32| Ok::<_, Error>(u64::from(value)),
                            ),
                            consumption: FrameConsumption::Exact,
                        }),
                        PayloadTypeCodec,
                    ),
                    minimum: 9,
                }
                .encoded_len(value.map.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: Tuple(
                OwnedString(ByteSizePrefix {
                    item: Utf8,
                    prefix: Mapped::new(
                        Number::<u32>::LE,
                        |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u32| Ok::<_, Error>(u64::from(value)),
                    ),
                    consumption: FrameConsumption::Exact,
                }),
                PayloadTypeCodec,
            ),
            minimum: 9,
        }
        .write(value.map.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Payload> for PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Payload, Error> {
        Ok(Payload {
            map: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Tuple(
                    OwnedString(ByteSizePrefix {
                        item: Utf8,
                        prefix: Mapped::new(
                            Number::<u32>::LE,
                            |value: &u64| u32::try_from(*value).map_err(|_| Error::Overflow),
                            |value: u32| Ok::<_, Error>(u64::from(value)),
                        ),
                        consumption: FrameConsumption::Exact,
                    }),
                    PayloadTypeCodec,
                ),
                minimum: 9,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `SeedsVec`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeedsVec {
    /// The `seeds` wire field.
    pub seeds: Vec<Vec<u8>>,
}
/// Bounded codec for [`SeedsVec`].
#[derive(Clone, Copy, Debug)]
pub struct SeedsVecCodec;
impl Encoder<SeedsVec> for SeedsVecCodec {
    fn encoded_len(&self, value: &SeedsVec) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: OwnedVector(CheckedCounted {
                        prefix: Number::<u32>::LE,
                        item: Number::<u8>::LE,
                        minimum: 1,
                    }),
                    minimum: 4,
                }
                .encoded_len(value.seeds.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SeedsVec, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: OwnedVector(CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            }),
            minimum: 4,
        }
        .write(value.seeds.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<SeedsVec> for SeedsVecCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SeedsVec, Error> {
        Ok(SeedsVec {
            seeds: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: OwnedVector(CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                }),
                minimum: 4,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `ProofInfo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofInfo {
    /// The `proof` wire field.
    pub proof: Vec<[u8; 32]>,
}
/// Bounded codec for [`ProofInfo`].
#[derive(Clone, Copy, Debug)]
pub struct ProofInfoCodec;
impl Encoder<ProofInfo> for ProofInfoCodec {
    fn encoded_len(&self, value: &ProofInfo) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: FixedBytes::<32>,
                    minimum: 32,
                }
                .encoded_len(value.proof.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ProofInfo, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: FixedBytes::<32>,
            minimum: 32,
        }
        .write(value.proof.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ProofInfo> for ProofInfoCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProofInfo, Error> {
        Ok(ProofInfo {
            proof: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: FixedBytes::<32>,
                minimum: 32,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `Reservation`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reservation {
    /// The `address` wire field.
    pub address: Address,
    /// The `spots_remaining` wire field.
    pub spots_remaining: u64,
    /// The `total_spots` wire field.
    pub total_spots: u64,
}
/// Bounded codec for [`Reservation`].
#[derive(Clone, Copy, Debug)]
pub struct ReservationCodec;
impl Encoder<Reservation> for ReservationCodec {
    fn encoded_len(&self, value: &Reservation) -> Result<usize, Error> {
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
            .checked_add(Number::<u64>::LE.encoded_len(&value.spots_remaining)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.total_spots)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Reservation, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Number::<u64>::LE.write(&value.spots_remaining, writer)?;
        Number::<u64>::LE.write(&value.total_spots, writer)?;
        Ok(())
    }
}
impl Decoder<Reservation> for ReservationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Reservation, Error> {
        Ok(Reservation {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            spots_remaining: Number::<u64>::LE.read(reader)?,
            total_spots: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `ReservationV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservationV1 {
    /// The `address` wire field.
    pub address: Address,
    /// The `spots_remaining` wire field.
    pub spots_remaining: u8,
    /// The `total_spots` wire field.
    pub total_spots: u8,
}
/// Bounded codec for [`ReservationV1`].
#[derive(Clone, Copy, Debug)]
pub struct ReservationV1Codec;
impl Encoder<ReservationV1> for ReservationV1Codec {
    fn encoded_len(&self, value: &ReservationV1) -> Result<usize, Error> {
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
            .checked_add(Number::<u8>::LE.encoded_len(&value.spots_remaining)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.total_spots)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ReservationV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.address, writer)?;
        Number::<u8>::LE.write(&value.spots_remaining, writer)?;
        Number::<u8>::LE.write(&value.total_spots, writer)?;
        Ok(())
    }
}
impl Decoder<ReservationV1> for ReservationV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ReservationV1, Error> {
        Ok(ReservationV1 {
            address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            spots_remaining: Number::<u8>::LE.read(reader)?,
            total_spots: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `SetCollectionSizeArgs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetCollectionSizeArgs {
    /// The `size` wire field.
    pub size: u64,
}
/// Bounded codec for [`SetCollectionSizeArgs`].
#[derive(Clone, Copy, Debug)]
pub struct SetCollectionSizeArgsCodec;
impl Encoder<SetCollectionSizeArgs> for SetCollectionSizeArgsCodec {
    fn encoded_len(&self, value: &SetCollectionSizeArgs) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetCollectionSizeArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.size, writer)?;
        Ok(())
    }
}
impl Decoder<SetCollectionSizeArgs> for SetCollectionSizeArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetCollectionSizeArgs, Error> {
        Ok(SetCollectionSizeArgs {
            size: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `MintNewEditionFromMasterEditionViaTokenArgs`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintNewEditionFromMasterEditionViaTokenArgs {
    /// The `edition` wire field.
    pub edition: u64,
}
/// Bounded codec for [`MintNewEditionFromMasterEditionViaTokenArgs`].
#[derive(Clone, Copy, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenArgsCodec;
impl Encoder<MintNewEditionFromMasterEditionViaTokenArgs>
    for MintNewEditionFromMasterEditionViaTokenArgsCodec
{
    fn encoded_len(
        &self,
        value: &MintNewEditionFromMasterEditionViaTokenArgs,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.edition)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MintNewEditionFromMasterEditionViaTokenArgs,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.edition, writer)?;
        Ok(())
    }
}
impl Decoder<MintNewEditionFromMasterEditionViaTokenArgs>
    for MintNewEditionFromMasterEditionViaTokenArgsCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<MintNewEditionFromMasterEditionViaTokenArgs, Error> {
        Ok(MintNewEditionFromMasterEditionViaTokenArgs {
            edition: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityType {
    /// The `None` variant.
    None,
    /// The `Metadata` variant.
    Metadata,
    /// The `Holder` variant.
    Holder,
    /// The `MetadataDelegate` variant.
    MetadataDelegate,
    /// The `TokenDelegate` variant.
    TokenDelegate,
}
/// Bounded codec for [`AuthorityType`].
#[derive(Clone, Copy, Debug)]
pub struct AuthorityTypeCodec;
const AUTHORITYTYPE_TAG_0: u8 = 0;
const AUTHORITYTYPE_TAG_1: u8 = 1;
const AUTHORITYTYPE_TAG_2: u8 = 2;
const AUTHORITYTYPE_TAG_3: u8 = 3;
const AUTHORITYTYPE_TAG_4: u8 = 4;
impl Encoder<AuthorityType> for AuthorityTypeCodec {
    fn encoded_len(&self, value: &AuthorityType) -> Result<usize, Error> {
        let size = match value {
            AuthorityType::None => 0usize,
            AuthorityType::Metadata => 0usize,
            AuthorityType::Holder => 0usize,
            AuthorityType::MetadataDelegate => 0usize,
            AuthorityType::TokenDelegate => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &AuthorityType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            AuthorityType::None => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_0, writer)?;
                Ok(())
            }
            AuthorityType::Metadata => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_1, writer)?;
                Ok(())
            }
            AuthorityType::Holder => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_2, writer)?;
                Ok(())
            }
            AuthorityType::MetadataDelegate => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_3, writer)?;
                Ok(())
            }
            AuthorityType::TokenDelegate => {
                Number::<u8>::LE.write(&AUTHORITYTYPE_TAG_4, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<AuthorityType> for AuthorityTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AuthorityType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            AUTHORITYTYPE_TAG_0 => Ok(AuthorityType::None),
            AUTHORITYTYPE_TAG_1 => Ok(AuthorityType::Metadata),
            AUTHORITYTYPE_TAG_2 => Ok(AuthorityType::Holder),
            AUTHORITYTYPE_TAG_3 => Ok(AuthorityType::MetadataDelegate),
            AUTHORITYTYPE_TAG_4 => Ok(AuthorityType::TokenDelegate),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HolderDelegateRole {
    /// The `PrintDelegate` variant.
    PrintDelegate,
}
/// Bounded codec for [`HolderDelegateRole`].
#[derive(Clone, Copy, Debug)]
pub struct HolderDelegateRoleCodec;
const HOLDERDELEGATEROLE_TAG_0: u8 = 0;
impl Encoder<HolderDelegateRole> for HolderDelegateRoleCodec {
    fn encoded_len(&self, value: &HolderDelegateRole) -> Result<usize, Error> {
        let size = match value {
            HolderDelegateRole::PrintDelegate => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &HolderDelegateRole, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            HolderDelegateRole::PrintDelegate => {
                Number::<u8>::LE.write(&HOLDERDELEGATEROLE_TAG_0, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<HolderDelegateRole> for HolderDelegateRoleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HolderDelegateRole, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            HOLDERDELEGATEROLE_TAG_0 => Ok(HolderDelegateRole::PrintDelegate),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    /// The `Uninitialized` variant.
    Uninitialized,
    /// The `EditionV1` variant.
    EditionV1,
    /// The `MasterEditionV1` variant.
    MasterEditionV1,
    /// The `ReservationListV1` variant.
    ReservationListV1,
    /// The `MetadataV1` variant.
    MetadataV1,
    /// The `ReservationListV2` variant.
    ReservationListV2,
    /// The `MasterEditionV2` variant.
    MasterEditionV2,
    /// The `EditionMarker` variant.
    EditionMarker,
    /// The `UseAuthorityRecord` variant.
    UseAuthorityRecord,
    /// The `CollectionAuthorityRecord` variant.
    CollectionAuthorityRecord,
    /// The `TokenOwnedEscrow` variant.
    TokenOwnedEscrow,
    /// The `TokenRecord` variant.
    TokenRecord,
    /// The `MetadataDelegate` variant.
    MetadataDelegate,
    /// The `EditionMarkerV2` variant.
    EditionMarkerV2,
    /// The `HolderDelegate` variant.
    HolderDelegate,
}
/// Bounded codec for [`Key`].
#[derive(Clone, Copy, Debug)]
pub struct KeyCodec;
const KEY_TAG_0: u8 = 0;
const KEY_TAG_1: u8 = 1;
const KEY_TAG_2: u8 = 2;
const KEY_TAG_3: u8 = 3;
const KEY_TAG_4: u8 = 4;
const KEY_TAG_5: u8 = 5;
const KEY_TAG_6: u8 = 6;
const KEY_TAG_7: u8 = 7;
const KEY_TAG_8: u8 = 8;
const KEY_TAG_9: u8 = 9;
const KEY_TAG_10: u8 = 10;
const KEY_TAG_11: u8 = 11;
const KEY_TAG_12: u8 = 12;
const KEY_TAG_13: u8 = 13;
const KEY_TAG_14: u8 = 14;
impl Encoder<Key> for KeyCodec {
    fn encoded_len(&self, value: &Key) -> Result<usize, Error> {
        let size = match value {
            Key::Uninitialized => 0usize,
            Key::EditionV1 => 0usize,
            Key::MasterEditionV1 => 0usize,
            Key::ReservationListV1 => 0usize,
            Key::MetadataV1 => 0usize,
            Key::ReservationListV2 => 0usize,
            Key::MasterEditionV2 => 0usize,
            Key::EditionMarker => 0usize,
            Key::UseAuthorityRecord => 0usize,
            Key::CollectionAuthorityRecord => 0usize,
            Key::TokenOwnedEscrow => 0usize,
            Key::TokenRecord => 0usize,
            Key::MetadataDelegate => 0usize,
            Key::EditionMarkerV2 => 0usize,
            Key::HolderDelegate => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &Key, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            Key::Uninitialized => {
                Number::<u8>::LE.write(&KEY_TAG_0, writer)?;
                Ok(())
            }
            Key::EditionV1 => {
                Number::<u8>::LE.write(&KEY_TAG_1, writer)?;
                Ok(())
            }
            Key::MasterEditionV1 => {
                Number::<u8>::LE.write(&KEY_TAG_2, writer)?;
                Ok(())
            }
            Key::ReservationListV1 => {
                Number::<u8>::LE.write(&KEY_TAG_3, writer)?;
                Ok(())
            }
            Key::MetadataV1 => {
                Number::<u8>::LE.write(&KEY_TAG_4, writer)?;
                Ok(())
            }
            Key::ReservationListV2 => {
                Number::<u8>::LE.write(&KEY_TAG_5, writer)?;
                Ok(())
            }
            Key::MasterEditionV2 => {
                Number::<u8>::LE.write(&KEY_TAG_6, writer)?;
                Ok(())
            }
            Key::EditionMarker => {
                Number::<u8>::LE.write(&KEY_TAG_7, writer)?;
                Ok(())
            }
            Key::UseAuthorityRecord => {
                Number::<u8>::LE.write(&KEY_TAG_8, writer)?;
                Ok(())
            }
            Key::CollectionAuthorityRecord => {
                Number::<u8>::LE.write(&KEY_TAG_9, writer)?;
                Ok(())
            }
            Key::TokenOwnedEscrow => {
                Number::<u8>::LE.write(&KEY_TAG_10, writer)?;
                Ok(())
            }
            Key::TokenRecord => {
                Number::<u8>::LE.write(&KEY_TAG_11, writer)?;
                Ok(())
            }
            Key::MetadataDelegate => {
                Number::<u8>::LE.write(&KEY_TAG_12, writer)?;
                Ok(())
            }
            Key::EditionMarkerV2 => {
                Number::<u8>::LE.write(&KEY_TAG_13, writer)?;
                Ok(())
            }
            Key::HolderDelegate => {
                Number::<u8>::LE.write(&KEY_TAG_14, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<Key> for KeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Key, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            KEY_TAG_0 => Ok(Key::Uninitialized),
            KEY_TAG_1 => Ok(Key::EditionV1),
            KEY_TAG_2 => Ok(Key::MasterEditionV1),
            KEY_TAG_3 => Ok(Key::ReservationListV1),
            KEY_TAG_4 => Ok(Key::MetadataV1),
            KEY_TAG_5 => Ok(Key::ReservationListV2),
            KEY_TAG_6 => Ok(Key::MasterEditionV2),
            KEY_TAG_7 => Ok(Key::EditionMarker),
            KEY_TAG_8 => Ok(Key::UseAuthorityRecord),
            KEY_TAG_9 => Ok(Key::CollectionAuthorityRecord),
            KEY_TAG_10 => Ok(Key::TokenOwnedEscrow),
            KEY_TAG_11 => Ok(Key::TokenRecord),
            KEY_TAG_12 => Ok(Key::MetadataDelegate),
            KEY_TAG_13 => Ok(Key::EditionMarkerV2),
            KEY_TAG_14 => Ok(Key::HolderDelegate),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MetadataDelegateRole {
    /// The `AuthorityItem` variant.
    AuthorityItem,
    /// The `Collection` variant.
    Collection,
    /// The `Use` variant.
    Use,
    /// The `Data` variant.
    Data,
    /// The `ProgrammableConfig` variant.
    ProgrammableConfig,
    /// The `DataItem` variant.
    DataItem,
    /// The `CollectionItem` variant.
    CollectionItem,
    /// The `ProgrammableConfigItem` variant.
    ProgrammableConfigItem,
}
/// Bounded codec for [`MetadataDelegateRole`].
#[derive(Clone, Copy, Debug)]
pub struct MetadataDelegateRoleCodec;
const METADATADELEGATEROLE_TAG_0: u8 = 0;
const METADATADELEGATEROLE_TAG_1: u8 = 1;
const METADATADELEGATEROLE_TAG_2: u8 = 2;
const METADATADELEGATEROLE_TAG_3: u8 = 3;
const METADATADELEGATEROLE_TAG_4: u8 = 4;
const METADATADELEGATEROLE_TAG_5: u8 = 5;
const METADATADELEGATEROLE_TAG_6: u8 = 6;
const METADATADELEGATEROLE_TAG_7: u8 = 7;
impl Encoder<MetadataDelegateRole> for MetadataDelegateRoleCodec {
    fn encoded_len(&self, value: &MetadataDelegateRole) -> Result<usize, Error> {
        let size = match value {
            MetadataDelegateRole::AuthorityItem => 0usize,
            MetadataDelegateRole::Collection => 0usize,
            MetadataDelegateRole::Use => 0usize,
            MetadataDelegateRole::Data => 0usize,
            MetadataDelegateRole::ProgrammableConfig => 0usize,
            MetadataDelegateRole::DataItem => 0usize,
            MetadataDelegateRole::CollectionItem => 0usize,
            MetadataDelegateRole::ProgrammableConfigItem => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &MetadataDelegateRole, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MetadataDelegateRole::AuthorityItem => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_0, writer)?;
                Ok(())
            }
            MetadataDelegateRole::Collection => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_1, writer)?;
                Ok(())
            }
            MetadataDelegateRole::Use => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_2, writer)?;
                Ok(())
            }
            MetadataDelegateRole::Data => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_3, writer)?;
                Ok(())
            }
            MetadataDelegateRole::ProgrammableConfig => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_4, writer)?;
                Ok(())
            }
            MetadataDelegateRole::DataItem => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_5, writer)?;
                Ok(())
            }
            MetadataDelegateRole::CollectionItem => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_6, writer)?;
                Ok(())
            }
            MetadataDelegateRole::ProgrammableConfigItem => {
                Number::<u8>::LE.write(&METADATADELEGATEROLE_TAG_7, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<MetadataDelegateRole> for MetadataDelegateRoleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MetadataDelegateRole, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            METADATADELEGATEROLE_TAG_0 => Ok(MetadataDelegateRole::AuthorityItem),
            METADATADELEGATEROLE_TAG_1 => Ok(MetadataDelegateRole::Collection),
            METADATADELEGATEROLE_TAG_2 => Ok(MetadataDelegateRole::Use),
            METADATADELEGATEROLE_TAG_3 => Ok(MetadataDelegateRole::Data),
            METADATADELEGATEROLE_TAG_4 => Ok(MetadataDelegateRole::ProgrammableConfig),
            METADATADELEGATEROLE_TAG_5 => Ok(MetadataDelegateRole::DataItem),
            METADATADELEGATEROLE_TAG_6 => Ok(MetadataDelegateRole::CollectionItem),
            METADATADELEGATEROLE_TAG_7 => Ok(MetadataDelegateRole::ProgrammableConfigItem),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationType {
    /// The `CollectionV1` variant.
    CollectionV1,
    /// The `ProgrammableV1` variant.
    ProgrammableV1,
}
/// Bounded codec for [`MigrationType`].
#[derive(Clone, Copy, Debug)]
pub struct MigrationTypeCodec;
const MIGRATIONTYPE_TAG_0: u8 = 0;
const MIGRATIONTYPE_TAG_1: u8 = 1;
impl Encoder<MigrationType> for MigrationTypeCodec {
    fn encoded_len(&self, value: &MigrationType) -> Result<usize, Error> {
        let size = match value {
            MigrationType::CollectionV1 => 0usize,
            MigrationType::ProgrammableV1 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &MigrationType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MigrationType::CollectionV1 => {
                Number::<u8>::LE.write(&MIGRATIONTYPE_TAG_0, writer)?;
                Ok(())
            }
            MigrationType::ProgrammableV1 => {
                Number::<u8>::LE.write(&MIGRATIONTYPE_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<MigrationType> for MigrationTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MigrationType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            MIGRATIONTYPE_TAG_0 => Ok(MigrationType::CollectionV1),
            MIGRATIONTYPE_TAG_1 => Ok(MigrationType::ProgrammableV1),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PayloadKey {
    /// The `Amount` variant.
    Amount,
    /// The `Authority` variant.
    Authority,
    /// The `AuthoritySeeds` variant.
    AuthoritySeeds,
    /// The `Delegate` variant.
    Delegate,
    /// The `DelegateSeeds` variant.
    DelegateSeeds,
    /// The `Destination` variant.
    Destination,
    /// The `DestinationSeeds` variant.
    DestinationSeeds,
    /// The `Holder` variant.
    Holder,
    /// The `Source` variant.
    Source,
    /// The `SourceSeeds` variant.
    SourceSeeds,
}
/// Bounded codec for [`PayloadKey`].
#[derive(Clone, Copy, Debug)]
pub struct PayloadKeyCodec;
const PAYLOADKEY_TAG_0: u8 = 0;
const PAYLOADKEY_TAG_1: u8 = 1;
const PAYLOADKEY_TAG_2: u8 = 2;
const PAYLOADKEY_TAG_3: u8 = 3;
const PAYLOADKEY_TAG_4: u8 = 4;
const PAYLOADKEY_TAG_5: u8 = 5;
const PAYLOADKEY_TAG_6: u8 = 6;
const PAYLOADKEY_TAG_7: u8 = 7;
const PAYLOADKEY_TAG_8: u8 = 8;
const PAYLOADKEY_TAG_9: u8 = 9;
impl Encoder<PayloadKey> for PayloadKeyCodec {
    fn encoded_len(&self, value: &PayloadKey) -> Result<usize, Error> {
        let size = match value {
            PayloadKey::Amount => 0usize,
            PayloadKey::Authority => 0usize,
            PayloadKey::AuthoritySeeds => 0usize,
            PayloadKey::Delegate => 0usize,
            PayloadKey::DelegateSeeds => 0usize,
            PayloadKey::Destination => 0usize,
            PayloadKey::DestinationSeeds => 0usize,
            PayloadKey::Holder => 0usize,
            PayloadKey::Source => 0usize,
            PayloadKey::SourceSeeds => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &PayloadKey, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            PayloadKey::Amount => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_0, writer)?;
                Ok(())
            }
            PayloadKey::Authority => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_1, writer)?;
                Ok(())
            }
            PayloadKey::AuthoritySeeds => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_2, writer)?;
                Ok(())
            }
            PayloadKey::Delegate => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_3, writer)?;
                Ok(())
            }
            PayloadKey::DelegateSeeds => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_4, writer)?;
                Ok(())
            }
            PayloadKey::Destination => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_5, writer)?;
                Ok(())
            }
            PayloadKey::DestinationSeeds => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_6, writer)?;
                Ok(())
            }
            PayloadKey::Holder => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_7, writer)?;
                Ok(())
            }
            PayloadKey::Source => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_8, writer)?;
                Ok(())
            }
            PayloadKey::SourceSeeds => {
                Number::<u8>::LE.write(&PAYLOADKEY_TAG_9, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<PayloadKey> for PayloadKeyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PayloadKey, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PAYLOADKEY_TAG_0 => Ok(PayloadKey::Amount),
            PAYLOADKEY_TAG_1 => Ok(PayloadKey::Authority),
            PAYLOADKEY_TAG_2 => Ok(PayloadKey::AuthoritySeeds),
            PAYLOADKEY_TAG_3 => Ok(PayloadKey::Delegate),
            PAYLOADKEY_TAG_4 => Ok(PayloadKey::DelegateSeeds),
            PAYLOADKEY_TAG_5 => Ok(PayloadKey::Destination),
            PAYLOADKEY_TAG_6 => Ok(PayloadKey::DestinationSeeds),
            PAYLOADKEY_TAG_7 => Ok(PayloadKey::Holder),
            PAYLOADKEY_TAG_8 => Ok(PayloadKey::Source),
            PAYLOADKEY_TAG_9 => Ok(PayloadKey::SourceSeeds),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RevokeArgs {
    /// The `CollectionV1` variant.
    CollectionV1,
    /// The `SaleV1` variant.
    SaleV1,
    /// The `TransferV1` variant.
    TransferV1,
    /// The `DataV1` variant.
    DataV1,
    /// The `UtilityV1` variant.
    UtilityV1,
    /// The `StakingV1` variant.
    StakingV1,
    /// The `StandardV1` variant.
    StandardV1,
    /// The `LockedTransferV1` variant.
    LockedTransferV1,
    /// The `ProgrammableConfigV1` variant.
    ProgrammableConfigV1,
    /// The `MigrationV1` variant.
    MigrationV1,
    /// The `AuthorityItemV1` variant.
    AuthorityItemV1,
    /// The `DataItemV1` variant.
    DataItemV1,
    /// The `CollectionItemV1` variant.
    CollectionItemV1,
    /// The `ProgrammableConfigItemV1` variant.
    ProgrammableConfigItemV1,
    /// The `PrintDelegateV1` variant.
    PrintDelegateV1,
}
/// Bounded codec for [`RevokeArgs`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeArgsCodec;
const REVOKEARGS_TAG_0: u8 = 0;
const REVOKEARGS_TAG_1: u8 = 1;
const REVOKEARGS_TAG_2: u8 = 2;
const REVOKEARGS_TAG_3: u8 = 3;
const REVOKEARGS_TAG_4: u8 = 4;
const REVOKEARGS_TAG_5: u8 = 5;
const REVOKEARGS_TAG_6: u8 = 6;
const REVOKEARGS_TAG_7: u8 = 7;
const REVOKEARGS_TAG_8: u8 = 8;
const REVOKEARGS_TAG_9: u8 = 9;
const REVOKEARGS_TAG_10: u8 = 10;
const REVOKEARGS_TAG_11: u8 = 11;
const REVOKEARGS_TAG_12: u8 = 12;
const REVOKEARGS_TAG_13: u8 = 13;
const REVOKEARGS_TAG_14: u8 = 14;
impl Encoder<RevokeArgs> for RevokeArgsCodec {
    fn encoded_len(&self, value: &RevokeArgs) -> Result<usize, Error> {
        let size = match value {
            RevokeArgs::CollectionV1 => 0usize,
            RevokeArgs::SaleV1 => 0usize,
            RevokeArgs::TransferV1 => 0usize,
            RevokeArgs::DataV1 => 0usize,
            RevokeArgs::UtilityV1 => 0usize,
            RevokeArgs::StakingV1 => 0usize,
            RevokeArgs::StandardV1 => 0usize,
            RevokeArgs::LockedTransferV1 => 0usize,
            RevokeArgs::ProgrammableConfigV1 => 0usize,
            RevokeArgs::MigrationV1 => 0usize,
            RevokeArgs::AuthorityItemV1 => 0usize,
            RevokeArgs::DataItemV1 => 0usize,
            RevokeArgs::CollectionItemV1 => 0usize,
            RevokeArgs::ProgrammableConfigItemV1 => 0usize,
            RevokeArgs::PrintDelegateV1 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &RevokeArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            RevokeArgs::CollectionV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_0, writer)?;
                Ok(())
            }
            RevokeArgs::SaleV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_1, writer)?;
                Ok(())
            }
            RevokeArgs::TransferV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_2, writer)?;
                Ok(())
            }
            RevokeArgs::DataV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_3, writer)?;
                Ok(())
            }
            RevokeArgs::UtilityV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_4, writer)?;
                Ok(())
            }
            RevokeArgs::StakingV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_5, writer)?;
                Ok(())
            }
            RevokeArgs::StandardV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_6, writer)?;
                Ok(())
            }
            RevokeArgs::LockedTransferV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_7, writer)?;
                Ok(())
            }
            RevokeArgs::ProgrammableConfigV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_8, writer)?;
                Ok(())
            }
            RevokeArgs::MigrationV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_9, writer)?;
                Ok(())
            }
            RevokeArgs::AuthorityItemV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_10, writer)?;
                Ok(())
            }
            RevokeArgs::DataItemV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_11, writer)?;
                Ok(())
            }
            RevokeArgs::CollectionItemV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_12, writer)?;
                Ok(())
            }
            RevokeArgs::ProgrammableConfigItemV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_13, writer)?;
                Ok(())
            }
            RevokeArgs::PrintDelegateV1 => {
                Number::<u8>::LE.write(&REVOKEARGS_TAG_14, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<RevokeArgs> for RevokeArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            REVOKEARGS_TAG_0 => Ok(RevokeArgs::CollectionV1),
            REVOKEARGS_TAG_1 => Ok(RevokeArgs::SaleV1),
            REVOKEARGS_TAG_2 => Ok(RevokeArgs::TransferV1),
            REVOKEARGS_TAG_3 => Ok(RevokeArgs::DataV1),
            REVOKEARGS_TAG_4 => Ok(RevokeArgs::UtilityV1),
            REVOKEARGS_TAG_5 => Ok(RevokeArgs::StakingV1),
            REVOKEARGS_TAG_6 => Ok(RevokeArgs::StandardV1),
            REVOKEARGS_TAG_7 => Ok(RevokeArgs::LockedTransferV1),
            REVOKEARGS_TAG_8 => Ok(RevokeArgs::ProgrammableConfigV1),
            REVOKEARGS_TAG_9 => Ok(RevokeArgs::MigrationV1),
            REVOKEARGS_TAG_10 => Ok(RevokeArgs::AuthorityItemV1),
            REVOKEARGS_TAG_11 => Ok(RevokeArgs::DataItemV1),
            REVOKEARGS_TAG_12 => Ok(RevokeArgs::CollectionItemV1),
            REVOKEARGS_TAG_13 => Ok(RevokeArgs::ProgrammableConfigItemV1),
            REVOKEARGS_TAG_14 => Ok(RevokeArgs::PrintDelegateV1),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenDelegateRole {
    /// The `Sale` variant.
    Sale,
    /// The `Transfer` variant.
    Transfer,
    /// The `Utility` variant.
    Utility,
    /// The `Staking` variant.
    Staking,
    /// The `Standard` variant.
    Standard,
    /// The `LockedTransfer` variant.
    LockedTransfer,
    /// The `Migration` variant.
    Migration,
}
/// Bounded codec for [`TokenDelegateRole`].
#[derive(Clone, Copy, Debug)]
pub struct TokenDelegateRoleCodec;
const TOKENDELEGATEROLE_TAG_0: u8 = 0;
const TOKENDELEGATEROLE_TAG_1: u8 = 1;
const TOKENDELEGATEROLE_TAG_2: u8 = 2;
const TOKENDELEGATEROLE_TAG_3: u8 = 3;
const TOKENDELEGATEROLE_TAG_4: u8 = 4;
const TOKENDELEGATEROLE_TAG_5: u8 = 5;
const TOKENDELEGATEROLE_TAG_6: u8 = 6;
impl Encoder<TokenDelegateRole> for TokenDelegateRoleCodec {
    fn encoded_len(&self, value: &TokenDelegateRole) -> Result<usize, Error> {
        let size = match value {
            TokenDelegateRole::Sale => 0usize,
            TokenDelegateRole::Transfer => 0usize,
            TokenDelegateRole::Utility => 0usize,
            TokenDelegateRole::Staking => 0usize,
            TokenDelegateRole::Standard => 0usize,
            TokenDelegateRole::LockedTransfer => 0usize,
            TokenDelegateRole::Migration => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &TokenDelegateRole, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TokenDelegateRole::Sale => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_0, writer)?;
                Ok(())
            }
            TokenDelegateRole::Transfer => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_1, writer)?;
                Ok(())
            }
            TokenDelegateRole::Utility => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_2, writer)?;
                Ok(())
            }
            TokenDelegateRole::Staking => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_3, writer)?;
                Ok(())
            }
            TokenDelegateRole::Standard => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_4, writer)?;
                Ok(())
            }
            TokenDelegateRole::LockedTransfer => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_5, writer)?;
                Ok(())
            }
            TokenDelegateRole::Migration => {
                Number::<u8>::LE.write(&TOKENDELEGATEROLE_TAG_6, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<TokenDelegateRole> for TokenDelegateRoleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenDelegateRole, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            TOKENDELEGATEROLE_TAG_0 => Ok(TokenDelegateRole::Sale),
            TOKENDELEGATEROLE_TAG_1 => Ok(TokenDelegateRole::Transfer),
            TOKENDELEGATEROLE_TAG_2 => Ok(TokenDelegateRole::Utility),
            TOKENDELEGATEROLE_TAG_3 => Ok(TokenDelegateRole::Staking),
            TOKENDELEGATEROLE_TAG_4 => Ok(TokenDelegateRole::Standard),
            TOKENDELEGATEROLE_TAG_5 => Ok(TokenDelegateRole::LockedTransfer),
            TOKENDELEGATEROLE_TAG_6 => Ok(TokenDelegateRole::Migration),
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
    /// The `ProgrammableNonFungible` variant.
    ProgrammableNonFungible,
    /// The `ProgrammableNonFungibleEdition` variant.
    ProgrammableNonFungibleEdition,
}
/// Bounded codec for [`TokenStandard`].
#[derive(Clone, Copy, Debug)]
pub struct TokenStandardCodec;
const TOKENSTANDARD_TAG_0: u8 = 0;
const TOKENSTANDARD_TAG_1: u8 = 1;
const TOKENSTANDARD_TAG_2: u8 = 2;
const TOKENSTANDARD_TAG_3: u8 = 3;
const TOKENSTANDARD_TAG_4: u8 = 4;
const TOKENSTANDARD_TAG_5: u8 = 5;
impl Encoder<TokenStandard> for TokenStandardCodec {
    fn encoded_len(&self, value: &TokenStandard) -> Result<usize, Error> {
        let size = match value {
            TokenStandard::NonFungible => 0usize,
            TokenStandard::FungibleAsset => 0usize,
            TokenStandard::Fungible => 0usize,
            TokenStandard::NonFungibleEdition => 0usize,
            TokenStandard::ProgrammableNonFungible => 0usize,
            TokenStandard::ProgrammableNonFungibleEdition => 0usize,
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
            TokenStandard::ProgrammableNonFungible => {
                Number::<u8>::LE.write(&TOKENSTANDARD_TAG_4, writer)?;
                Ok(())
            }
            TokenStandard::ProgrammableNonFungibleEdition => {
                Number::<u8>::LE.write(&TOKENSTANDARD_TAG_5, writer)?;
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
            TOKENSTANDARD_TAG_4 => Ok(TokenStandard::ProgrammableNonFungible),
            TOKENSTANDARD_TAG_5 => Ok(TokenStandard::ProgrammableNonFungibleEdition),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenState {
    /// The `Unlocked` variant.
    Unlocked,
    /// The `Locked` variant.
    Locked,
    /// The `Listed` variant.
    Listed,
}
/// Bounded codec for [`TokenState`].
#[derive(Clone, Copy, Debug)]
pub struct TokenStateCodec;
const TOKENSTATE_TAG_0: u8 = 0;
const TOKENSTATE_TAG_1: u8 = 1;
const TOKENSTATE_TAG_2: u8 = 2;
impl Encoder<TokenState> for TokenStateCodec {
    fn encoded_len(&self, value: &TokenState) -> Result<usize, Error> {
        let size = match value {
            TokenState::Unlocked => 0usize,
            TokenState::Locked => 0usize,
            TokenState::Listed => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &TokenState, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TokenState::Unlocked => {
                Number::<u8>::LE.write(&TOKENSTATE_TAG_0, writer)?;
                Ok(())
            }
            TokenState::Locked => {
                Number::<u8>::LE.write(&TOKENSTATE_TAG_1, writer)?;
                Ok(())
            }
            TokenState::Listed => {
                Number::<u8>::LE.write(&TOKENSTATE_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<TokenState> for TokenStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenState, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            TOKENSTATE_TAG_0 => Ok(TokenState::Unlocked),
            TOKENSTATE_TAG_1 => Ok(TokenState::Locked),
            TOKENSTATE_TAG_2 => Ok(TokenState::Listed),
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
pub enum VerificationArgs {
    /// The `CreatorV1` variant.
    CreatorV1,
    /// The `CollectionV1` variant.
    CollectionV1,
}
/// Bounded codec for [`VerificationArgs`].
#[derive(Clone, Copy, Debug)]
pub struct VerificationArgsCodec;
const VERIFICATIONARGS_TAG_0: u8 = 0;
const VERIFICATIONARGS_TAG_1: u8 = 1;
impl Encoder<VerificationArgs> for VerificationArgsCodec {
    fn encoded_len(&self, value: &VerificationArgs) -> Result<usize, Error> {
        let size = match value {
            VerificationArgs::CreatorV1 => 0usize,
            VerificationArgs::CollectionV1 => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &VerificationArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            VerificationArgs::CreatorV1 => {
                Number::<u8>::LE.write(&VERIFICATIONARGS_TAG_0, writer)?;
                Ok(())
            }
            VerificationArgs::CollectionV1 => {
                Number::<u8>::LE.write(&VERIFICATIONARGS_TAG_1, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<VerificationArgs> for VerificationArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerificationArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            VERIFICATIONARGS_TAG_0 => Ok(VerificationArgs::CreatorV1),
            VERIFICATIONARGS_TAG_1 => Ok(VerificationArgs::CollectionV1),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `BurnArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnArgsV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`BurnArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct BurnArgsV1PayloadCodec;
impl Encoder<BurnArgsV1Payload> for BurnArgsV1PayloadCodec {
    fn encoded_len(&self, value: &BurnArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &BurnArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<BurnArgsV1Payload> for BurnArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnArgsV1Payload, Error> {
        Ok(BurnArgsV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BurnArgs {
    /// The `V1` variant.
    V1(BurnArgsV1Payload),
}
/// Bounded codec for [`BurnArgs`].
#[derive(Clone, Copy, Debug)]
pub struct BurnArgsCodec;
const BURNARGS_TAG_0: u8 = 0;
impl Encoder<BurnArgs> for BurnArgsCodec {
    fn encoded_len(&self, value: &BurnArgs) -> Result<usize, Error> {
        let size = match value {
            BurnArgs::V1(value) => BurnArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &BurnArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            BurnArgs::V1(value) => {
                Number::<u8>::LE.write(&BURNARGS_TAG_0, writer)?;
                BurnArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<BurnArgs> for BurnArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            BURNARGS_TAG_0 => Ok(BurnArgs::V1(BurnArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `CollectionDetailsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionDetailsV1Payload {
    /// The `size` wire field.
    pub size: u64,
}
/// Bounded codec for [`CollectionDetailsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionDetailsV1PayloadCodec;
impl Encoder<CollectionDetailsV1Payload> for CollectionDetailsV1PayloadCodec {
    fn encoded_len(&self, value: &CollectionDetailsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.size)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CollectionDetailsV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.size, writer)?;
        Ok(())
    }
}
impl Decoder<CollectionDetailsV1Payload> for CollectionDetailsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionDetailsV1Payload, Error> {
        Ok(CollectionDetailsV1Payload {
            size: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `CollectionDetailsV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionDetailsV2Payload {
    /// The `padding` wire field.
    pub padding: [u8; 8],
}
/// Bounded codec for [`CollectionDetailsV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionDetailsV2PayloadCodec;
impl Encoder<CollectionDetailsV2Payload> for CollectionDetailsV2PayloadCodec {
    fn encoded_len(&self, value: &CollectionDetailsV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(FixedBytes::<8>.encoded_len(&value.padding)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CollectionDetailsV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        FixedBytes::<8>.write(&value.padding, writer)?;
        Ok(())
    }
}
impl Decoder<CollectionDetailsV2Payload> for CollectionDetailsV2PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionDetailsV2Payload, Error> {
        Ok(CollectionDetailsV2Payload {
            padding: FixedBytes::<8>.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionDetails {
    /// The `V1` variant.
    V1(CollectionDetailsV1Payload),
    /// The `V2` variant.
    V2(CollectionDetailsV2Payload),
}
/// Bounded codec for [`CollectionDetails`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionDetailsCodec;
const COLLECTIONDETAILS_TAG_0: u8 = 0;
const COLLECTIONDETAILS_TAG_1: u8 = 1;
impl Encoder<CollectionDetails> for CollectionDetailsCodec {
    fn encoded_len(&self, value: &CollectionDetails) -> Result<usize, Error> {
        let size = match value {
            CollectionDetails::V1(value) => CollectionDetailsV1PayloadCodec.encoded_len(value)?,
            CollectionDetails::V2(value) => CollectionDetailsV2PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &CollectionDetails, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            CollectionDetails::V1(value) => {
                Number::<u8>::LE.write(&COLLECTIONDETAILS_TAG_0, writer)?;
                CollectionDetailsV1PayloadCodec.write(value, writer)
            }
            CollectionDetails::V2(value) => {
                Number::<u8>::LE.write(&COLLECTIONDETAILS_TAG_1, writer)?;
                CollectionDetailsV2PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<CollectionDetails> for CollectionDetailsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionDetails, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            COLLECTIONDETAILS_TAG_0 => Ok(CollectionDetails::V1(
                CollectionDetailsV1PayloadCodec.read(reader)?,
            )),
            COLLECTIONDETAILS_TAG_1 => Ok(CollectionDetails::V2(
                CollectionDetailsV2PayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionDetailsToggle {
    /// The `None` variant.
    None,
    /// The `Clear` variant.
    Clear,
    /// The `Set` variant.
    Set(CollectionDetails),
}
/// Bounded codec for [`CollectionDetailsToggle`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionDetailsToggleCodec;
const COLLECTIONDETAILSTOGGLE_TAG_0: u8 = 0;
const COLLECTIONDETAILSTOGGLE_TAG_1: u8 = 1;
const COLLECTIONDETAILSTOGGLE_TAG_2: u8 = 2;
impl Encoder<CollectionDetailsToggle> for CollectionDetailsToggleCodec {
    fn encoded_len(&self, value: &CollectionDetailsToggle) -> Result<usize, Error> {
        let size = match value {
            CollectionDetailsToggle::None => 0usize,
            CollectionDetailsToggle::Clear => 0usize,
            CollectionDetailsToggle::Set(value) => CollectionDetailsCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &CollectionDetailsToggle, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            CollectionDetailsToggle::None => {
                Number::<u8>::LE.write(&COLLECTIONDETAILSTOGGLE_TAG_0, writer)?;
                Ok(())
            }
            CollectionDetailsToggle::Clear => {
                Number::<u8>::LE.write(&COLLECTIONDETAILSTOGGLE_TAG_1, writer)?;
                Ok(())
            }
            CollectionDetailsToggle::Set(value) => {
                Number::<u8>::LE.write(&COLLECTIONDETAILSTOGGLE_TAG_2, writer)?;
                CollectionDetailsCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<CollectionDetailsToggle> for CollectionDetailsToggleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionDetailsToggle, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            COLLECTIONDETAILSTOGGLE_TAG_0 => Ok(CollectionDetailsToggle::None),
            COLLECTIONDETAILSTOGGLE_TAG_1 => Ok(CollectionDetailsToggle::Clear),
            COLLECTIONDETAILSTOGGLE_TAG_2 => Ok(CollectionDetailsToggle::Set(
                CollectionDetailsCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectionToggle {
    /// The `None` variant.
    None,
    /// The `Clear` variant.
    Clear,
    /// The `Set` variant.
    Set(Collection),
}
/// Bounded codec for [`CollectionToggle`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionToggleCodec;
const COLLECTIONTOGGLE_TAG_0: u8 = 0;
const COLLECTIONTOGGLE_TAG_1: u8 = 1;
const COLLECTIONTOGGLE_TAG_2: u8 = 2;
impl Encoder<CollectionToggle> for CollectionToggleCodec {
    fn encoded_len(&self, value: &CollectionToggle) -> Result<usize, Error> {
        let size = match value {
            CollectionToggle::None => 0usize,
            CollectionToggle::Clear => 0usize,
            CollectionToggle::Set(value) => CollectionCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &CollectionToggle, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            CollectionToggle::None => {
                Number::<u8>::LE.write(&COLLECTIONTOGGLE_TAG_0, writer)?;
                Ok(())
            }
            CollectionToggle::Clear => {
                Number::<u8>::LE.write(&COLLECTIONTOGGLE_TAG_1, writer)?;
                Ok(())
            }
            CollectionToggle::Set(value) => {
                Number::<u8>::LE.write(&COLLECTIONTOGGLE_TAG_2, writer)?;
                CollectionCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<CollectionToggle> for CollectionToggleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionToggle, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            COLLECTIONTOGGLE_TAG_0 => Ok(CollectionToggle::None),
            COLLECTIONTOGGLE_TAG_1 => Ok(CollectionToggle::Clear),
            COLLECTIONTOGGLE_TAG_2 => Ok(CollectionToggle::Set(CollectionCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `CreateArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateArgsV1Payload {
    /// The `asset_data` wire field.
    pub asset_data: AssetData,
    /// The `decimals` wire field.
    pub decimals: Option<u8>,
    /// The `print_supply` wire field.
    pub print_supply: Option<PrintSupply>,
}
/// Bounded codec for [`CreateArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct CreateArgsV1PayloadCodec;
impl Encoder<CreateArgsV1Payload> for CreateArgsV1PayloadCodec {
    fn encoded_len(&self, value: &CreateArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(AssetDataCodec.encoded_len(&value.asset_data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u8>::LE).encoded_len(&value.decimals)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(PrintSupplyCodec).encoded_len(&value.print_supply)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        AssetDataCodec.write(&value.asset_data, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.decimals, writer)?;
        OptionCodec::u8(PrintSupplyCodec).write(&value.print_supply, writer)?;
        Ok(())
    }
}
impl Decoder<CreateArgsV1Payload> for CreateArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateArgsV1Payload, Error> {
        Ok(CreateArgsV1Payload {
            asset_data: AssetDataCodec.read(reader)?,
            decimals: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            print_supply: OptionCodec::u8(PrintSupplyCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreateArgs {
    /// The `V1` variant.
    V1(CreateArgsV1Payload),
}
/// Bounded codec for [`CreateArgs`].
#[derive(Clone, Copy, Debug)]
pub struct CreateArgsCodec;
const CREATEARGS_TAG_0: u8 = 0;
impl Encoder<CreateArgs> for CreateArgsCodec {
    fn encoded_len(&self, value: &CreateArgs) -> Result<usize, Error> {
        let size = match value {
            CreateArgs::V1(value) => CreateArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &CreateArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            CreateArgs::V1(value) => {
                Number::<u8>::LE.write(&CREATEARGS_TAG_0, writer)?;
                CreateArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<CreateArgs> for CreateArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            CREATEARGS_TAG_0 => Ok(CreateArgs::V1(CreateArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowAuthority {
    /// The `TokenOwner` variant.
    TokenOwner,
    /// The `Creator` variant.
    Creator(Address),
}
/// Bounded codec for [`EscrowAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct EscrowAuthorityCodec;
const ESCROWAUTHORITY_TAG_0: u8 = 0;
const ESCROWAUTHORITY_TAG_1: u8 = 1;
impl Encoder<EscrowAuthority> for EscrowAuthorityCodec {
    fn encoded_len(&self, value: &EscrowAuthority) -> Result<usize, Error> {
        let size = match value {
            EscrowAuthority::TokenOwner => 0usize,
            EscrowAuthority::Creator(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &EscrowAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            EscrowAuthority::TokenOwner => {
                Number::<u8>::LE.write(&ESCROWAUTHORITY_TAG_0, writer)?;
                Ok(())
            }
            EscrowAuthority::Creator(value) => {
                Number::<u8>::LE.write(&ESCROWAUTHORITY_TAG_1, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
        }
    }
}
impl Decoder<EscrowAuthority> for EscrowAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EscrowAuthority, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            ESCROWAUTHORITY_TAG_0 => Ok(EscrowAuthority::TokenOwner),
            ESCROWAUTHORITY_TAG_1 => Ok(EscrowAuthority::Creator(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `LockArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockArgsV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`LockArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct LockArgsV1PayloadCodec;
impl Encoder<LockArgsV1Payload> for LockArgsV1PayloadCodec {
    fn encoded_len(&self, value: &LockArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &LockArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<LockArgsV1Payload> for LockArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LockArgsV1Payload, Error> {
        Ok(LockArgsV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LockArgs {
    /// The `V1` variant.
    V1(LockArgsV1Payload),
}
/// Bounded codec for [`LockArgs`].
#[derive(Clone, Copy, Debug)]
pub struct LockArgsCodec;
const LOCKARGS_TAG_0: u8 = 0;
impl Encoder<LockArgs> for LockArgsCodec {
    fn encoded_len(&self, value: &LockArgs) -> Result<usize, Error> {
        let size = match value {
            LockArgs::V1(value) => LockArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &LockArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            LockArgs::V1(value) => {
                Number::<u8>::LE.write(&LOCKARGS_TAG_0, writer)?;
                LockArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<LockArgs> for LockArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<LockArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            LOCKARGS_TAG_0 => Ok(LockArgs::V1(LockArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `UnlockArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnlockArgsV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UnlockArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UnlockArgsV1PayloadCodec;
impl Encoder<UnlockArgsV1Payload> for UnlockArgsV1PayloadCodec {
    fn encoded_len(&self, value: &UnlockArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UnlockArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UnlockArgsV1Payload> for UnlockArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnlockArgsV1Payload, Error> {
        Ok(UnlockArgsV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnlockArgs {
    /// The `V1` variant.
    V1(UnlockArgsV1Payload),
}
/// Bounded codec for [`UnlockArgs`].
#[derive(Clone, Copy, Debug)]
pub struct UnlockArgsCodec;
const UNLOCKARGS_TAG_0: u8 = 0;
impl Encoder<UnlockArgs> for UnlockArgsCodec {
    fn encoded_len(&self, value: &UnlockArgs) -> Result<usize, Error> {
        let size = match value {
            UnlockArgs::V1(value) => UnlockArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UnlockArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UnlockArgs::V1(value) => {
                Number::<u8>::LE.write(&UNLOCKARGS_TAG_0, writer)?;
                UnlockArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<UnlockArgs> for UnlockArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnlockArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            UNLOCKARGS_TAG_0 => Ok(UnlockArgs::V1(UnlockArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `UseArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UseArgsV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UseArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UseArgsV1PayloadCodec;
impl Encoder<UseArgsV1Payload> for UseArgsV1PayloadCodec {
    fn encoded_len(&self, value: &UseArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UseArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UseArgsV1Payload> for UseArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UseArgsV1Payload, Error> {
        Ok(UseArgsV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UseArgs {
    /// The `V1` variant.
    V1(UseArgsV1Payload),
}
/// Bounded codec for [`UseArgs`].
#[derive(Clone, Copy, Debug)]
pub struct UseArgsCodec;
const USEARGS_TAG_0: u8 = 0;
impl Encoder<UseArgs> for UseArgsCodec {
    fn encoded_len(&self, value: &UseArgs) -> Result<usize, Error> {
        let size = match value {
            UseArgs::V1(value) => UseArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UseArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UseArgs::V1(value) => {
                Number::<u8>::LE.write(&USEARGS_TAG_0, writer)?;
                UseArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<UseArgs> for UseArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UseArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            USEARGS_TAG_0 => Ok(UseArgs::V1(UseArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `MintArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintArgsV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`MintArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct MintArgsV1PayloadCodec;
impl Encoder<MintArgsV1Payload> for MintArgsV1PayloadCodec {
    fn encoded_len(&self, value: &MintArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MintArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<MintArgsV1Payload> for MintArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintArgsV1Payload, Error> {
        Ok(MintArgsV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MintArgs {
    /// The `V1` variant.
    V1(MintArgsV1Payload),
}
/// Bounded codec for [`MintArgs`].
#[derive(Clone, Copy, Debug)]
pub struct MintArgsCodec;
const MINTARGS_TAG_0: u8 = 0;
impl Encoder<MintArgs> for MintArgsCodec {
    fn encoded_len(&self, value: &MintArgs) -> Result<usize, Error> {
        let size = match value {
            MintArgs::V1(value) => MintArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &MintArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            MintArgs::V1(value) => {
                Number::<u8>::LE.write(&MINTARGS_TAG_0, writer)?;
                MintArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<MintArgs> for MintArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MintArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            MINTARGS_TAG_0 => Ok(MintArgs::V1(MintArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `TransferArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferArgsV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`TransferArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct TransferArgsV1PayloadCodec;
impl Encoder<TransferArgsV1Payload> for TransferArgsV1PayloadCodec {
    fn encoded_len(&self, value: &TransferArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<TransferArgsV1Payload> for TransferArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferArgsV1Payload, Error> {
        Ok(TransferArgsV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransferArgs {
    /// The `V1` variant.
    V1(TransferArgsV1Payload),
}
/// Bounded codec for [`TransferArgs`].
#[derive(Clone, Copy, Debug)]
pub struct TransferArgsCodec;
const TRANSFERARGS_TAG_0: u8 = 0;
impl Encoder<TransferArgs> for TransferArgsCodec {
    fn encoded_len(&self, value: &TransferArgs) -> Result<usize, Error> {
        let size = match value {
            TransferArgs::V1(value) => TransferArgsV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &TransferArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            TransferArgs::V1(value) => {
                Number::<u8>::LE.write(&TRANSFERARGS_TAG_0, writer)?;
                TransferArgsV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<TransferArgs> for TransferArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            TRANSFERARGS_TAG_0 => Ok(TransferArgs::V1(TransferArgsV1PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PayloadType {
    /// The `Pubkey` variant.
    Pubkey(Address),
    /// The `Seeds` variant.
    Seeds(SeedsVec),
    /// The `MerkleProof` variant.
    MerkleProof(ProofInfo),
    /// The `Number` variant.
    Number(u64),
}
/// Bounded codec for [`PayloadType`].
#[derive(Clone, Copy, Debug)]
pub struct PayloadTypeCodec;
const PAYLOADTYPE_TAG_0: u8 = 0;
const PAYLOADTYPE_TAG_1: u8 = 1;
const PAYLOADTYPE_TAG_2: u8 = 2;
const PAYLOADTYPE_TAG_3: u8 = 3;
impl Encoder<PayloadType> for PayloadTypeCodec {
    fn encoded_len(&self, value: &PayloadType) -> Result<usize, Error> {
        let size = match value {
            PayloadType::Pubkey(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
            PayloadType::Seeds(value) => SeedsVecCodec.encoded_len(value)?,
            PayloadType::MerkleProof(value) => ProofInfoCodec.encoded_len(value)?,
            PayloadType::Number(value) => Number::<u64>::LE.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &PayloadType, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            PayloadType::Pubkey(value) => {
                Number::<u8>::LE.write(&PAYLOADTYPE_TAG_0, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
            PayloadType::Seeds(value) => {
                Number::<u8>::LE.write(&PAYLOADTYPE_TAG_1, writer)?;
                SeedsVecCodec.write(value, writer)
            }
            PayloadType::MerkleProof(value) => {
                Number::<u8>::LE.write(&PAYLOADTYPE_TAG_2, writer)?;
                ProofInfoCodec.write(value, writer)
            }
            PayloadType::Number(value) => {
                Number::<u8>::LE.write(&PAYLOADTYPE_TAG_3, writer)?;
                Number::<u64>::LE.write(value, writer)
            }
        }
    }
}
impl Decoder<PayloadType> for PayloadTypeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PayloadType, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PAYLOADTYPE_TAG_0 => Ok(PayloadType::Pubkey(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            PAYLOADTYPE_TAG_1 => Ok(PayloadType::Seeds(SeedsVecCodec.read(reader)?)),
            PAYLOADTYPE_TAG_2 => Ok(PayloadType::MerkleProof(ProofInfoCodec.read(reader)?)),
            PAYLOADTYPE_TAG_3 => Ok(PayloadType::Number(Number::<u64>::LE.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `PrintArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintArgsV1Payload {
    /// The `edition` wire field.
    pub edition: u64,
}
/// Bounded codec for [`PrintArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct PrintArgsV1PayloadCodec;
impl Encoder<PrintArgsV1Payload> for PrintArgsV1PayloadCodec {
    fn encoded_len(&self, value: &PrintArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.edition)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PrintArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.edition, writer)?;
        Ok(())
    }
}
impl Decoder<PrintArgsV1Payload> for PrintArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PrintArgsV1Payload, Error> {
        Ok(PrintArgsV1Payload {
            edition: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `PrintArgsV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintArgsV2Payload {
    /// The `edition` wire field.
    pub edition: u64,
}
/// Bounded codec for [`PrintArgsV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct PrintArgsV2PayloadCodec;
impl Encoder<PrintArgsV2Payload> for PrintArgsV2PayloadCodec {
    fn encoded_len(&self, value: &PrintArgsV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.edition)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &PrintArgsV2Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.edition, writer)?;
        Ok(())
    }
}
impl Decoder<PrintArgsV2Payload> for PrintArgsV2PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PrintArgsV2Payload, Error> {
        Ok(PrintArgsV2Payload {
            edition: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrintArgs {
    /// The `V1` variant.
    V1(PrintArgsV1Payload),
    /// The `V2` variant.
    V2(PrintArgsV2Payload),
}
/// Bounded codec for [`PrintArgs`].
#[derive(Clone, Copy, Debug)]
pub struct PrintArgsCodec;
const PRINTARGS_TAG_0: u8 = 0;
const PRINTARGS_TAG_1: u8 = 1;
impl Encoder<PrintArgs> for PrintArgsCodec {
    fn encoded_len(&self, value: &PrintArgs) -> Result<usize, Error> {
        let size = match value {
            PrintArgs::V1(value) => PrintArgsV1PayloadCodec.encoded_len(value)?,
            PrintArgs::V2(value) => PrintArgsV2PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &PrintArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            PrintArgs::V1(value) => {
                Number::<u8>::LE.write(&PRINTARGS_TAG_0, writer)?;
                PrintArgsV1PayloadCodec.write(value, writer)
            }
            PrintArgs::V2(value) => {
                Number::<u8>::LE.write(&PRINTARGS_TAG_1, writer)?;
                PrintArgsV2PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<PrintArgs> for PrintArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PrintArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PRINTARGS_TAG_0 => Ok(PrintArgs::V1(PrintArgsV1PayloadCodec.read(reader)?)),
            PRINTARGS_TAG_1 => Ok(PrintArgs::V2(PrintArgsV2PayloadCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrintSupply {
    /// The `Zero` variant.
    Zero,
    /// The `Limited` variant.
    Limited(u64),
    /// The `Unlimited` variant.
    Unlimited,
}
/// Bounded codec for [`PrintSupply`].
#[derive(Clone, Copy, Debug)]
pub struct PrintSupplyCodec;
const PRINTSUPPLY_TAG_0: u8 = 0;
const PRINTSUPPLY_TAG_1: u8 = 1;
const PRINTSUPPLY_TAG_2: u8 = 2;
impl Encoder<PrintSupply> for PrintSupplyCodec {
    fn encoded_len(&self, value: &PrintSupply) -> Result<usize, Error> {
        let size = match value {
            PrintSupply::Zero => 0usize,
            PrintSupply::Limited(value) => Number::<u64>::LE.encoded_len(value)?,
            PrintSupply::Unlimited => 0usize,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &PrintSupply, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            PrintSupply::Zero => {
                Number::<u8>::LE.write(&PRINTSUPPLY_TAG_0, writer)?;
                Ok(())
            }
            PrintSupply::Limited(value) => {
                Number::<u8>::LE.write(&PRINTSUPPLY_TAG_1, writer)?;
                Number::<u64>::LE.write(value, writer)
            }
            PrintSupply::Unlimited => {
                Number::<u8>::LE.write(&PRINTSUPPLY_TAG_2, writer)?;
                Ok(())
            }
        }
    }
}
impl Decoder<PrintSupply> for PrintSupplyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PrintSupply, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PRINTSUPPLY_TAG_0 => Ok(PrintSupply::Zero),
            PRINTSUPPLY_TAG_1 => Ok(PrintSupply::Limited(Number::<u64>::LE.read(reader)?)),
            PRINTSUPPLY_TAG_2 => Ok(PrintSupply::Unlimited),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `ProgrammableConfigV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgrammableConfigV1Payload {
    /// The `rule_set` wire field.
    pub rule_set: Option<Address>,
}
/// Bounded codec for [`ProgrammableConfigV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct ProgrammableConfigV1PayloadCodec;
impl Encoder<ProgrammableConfigV1Payload> for ProgrammableConfigV1PayloadCodec {
    fn encoded_len(&self, value: &ProgrammableConfigV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.rule_set)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &ProgrammableConfigV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.rule_set, writer)?;
        Ok(())
    }
}
impl Decoder<ProgrammableConfigV1Payload> for ProgrammableConfigV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgrammableConfigV1Payload, Error> {
        Ok(ProgrammableConfigV1Payload {
            rule_set: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProgrammableConfig {
    /// The `V1` variant.
    V1(ProgrammableConfigV1Payload),
}
/// Bounded codec for [`ProgrammableConfig`].
#[derive(Clone, Copy, Debug)]
pub struct ProgrammableConfigCodec;
const PROGRAMMABLECONFIG_TAG_0: u8 = 0;
impl Encoder<ProgrammableConfig> for ProgrammableConfigCodec {
    fn encoded_len(&self, value: &ProgrammableConfig) -> Result<usize, Error> {
        let size = match value {
            ProgrammableConfig::V1(value) => ProgrammableConfigV1PayloadCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ProgrammableConfig, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            ProgrammableConfig::V1(value) => {
                Number::<u8>::LE.write(&PROGRAMMABLECONFIG_TAG_0, writer)?;
                ProgrammableConfigV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<ProgrammableConfig> for ProgrammableConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgrammableConfig, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            PROGRAMMABLECONFIG_TAG_0 => Ok(ProgrammableConfig::V1(
                ProgrammableConfigV1PayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleSetToggle {
    /// The `None` variant.
    None,
    /// The `Clear` variant.
    Clear,
    /// The `Set` variant.
    Set(Address),
}
/// Bounded codec for [`RuleSetToggle`].
#[derive(Clone, Copy, Debug)]
pub struct RuleSetToggleCodec;
const RULESETTOGGLE_TAG_0: u8 = 0;
const RULESETTOGGLE_TAG_1: u8 = 1;
const RULESETTOGGLE_TAG_2: u8 = 2;
impl Encoder<RuleSetToggle> for RuleSetToggleCodec {
    fn encoded_len(&self, value: &RuleSetToggle) -> Result<usize, Error> {
        let size = match value {
            RuleSetToggle::None => 0usize,
            RuleSetToggle::Clear => 0usize,
            RuleSetToggle::Set(value) => Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &RuleSetToggle, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            RuleSetToggle::None => {
                Number::<u8>::LE.write(&RULESETTOGGLE_TAG_0, writer)?;
                Ok(())
            }
            RuleSetToggle::Clear => {
                Number::<u8>::LE.write(&RULESETTOGGLE_TAG_1, writer)?;
                Ok(())
            }
            RuleSetToggle::Set(value) => {
                Number::<u8>::LE.write(&RULESETTOGGLE_TAG_2, writer)?;
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .write(value, writer)
            }
        }
    }
}
impl Decoder<RuleSetToggle> for RuleSetToggleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RuleSetToggle, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            RULESETTOGGLE_TAG_0 => Ok(RuleSetToggle::None),
            RULESETTOGGLE_TAG_1 => Ok(RuleSetToggle::Clear),
            RULESETTOGGLE_TAG_2 => Ok(RuleSetToggle::Set(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UsesToggle {
    /// The `None` variant.
    None,
    /// The `Clear` variant.
    Clear,
    /// The `Set` variant.
    Set(Uses),
}
/// Bounded codec for [`UsesToggle`].
#[derive(Clone, Copy, Debug)]
pub struct UsesToggleCodec;
const USESTOGGLE_TAG_0: u8 = 0;
const USESTOGGLE_TAG_1: u8 = 1;
const USESTOGGLE_TAG_2: u8 = 2;
impl Encoder<UsesToggle> for UsesToggleCodec {
    fn encoded_len(&self, value: &UsesToggle) -> Result<usize, Error> {
        let size = match value {
            UsesToggle::None => 0usize,
            UsesToggle::Clear => 0usize,
            UsesToggle::Set(value) => UsesCodec.encoded_len(value)?,
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UsesToggle, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UsesToggle::None => {
                Number::<u8>::LE.write(&USESTOGGLE_TAG_0, writer)?;
                Ok(())
            }
            UsesToggle::Clear => {
                Number::<u8>::LE.write(&USESTOGGLE_TAG_1, writer)?;
                Ok(())
            }
            UsesToggle::Set(value) => {
                Number::<u8>::LE.write(&USESTOGGLE_TAG_2, writer)?;
                UsesCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<UsesToggle> for UsesToggleCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UsesToggle, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            USESTOGGLE_TAG_0 => Ok(UsesToggle::None),
            USESTOGGLE_TAG_1 => Ok(UsesToggle::Clear),
            USESTOGGLE_TAG_2 => Ok(UsesToggle::Set(UsesCodec.read(reader)?)),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `DelegateArgsCollectionV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsCollectionV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsCollectionV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsCollectionV1PayloadCodec;
impl Encoder<DelegateArgsCollectionV1Payload> for DelegateArgsCollectionV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsCollectionV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsCollectionV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsCollectionV1Payload> for DelegateArgsCollectionV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsCollectionV1Payload, Error> {
        Ok(DelegateArgsCollectionV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsSaleV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsSaleV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsSaleV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsSaleV1PayloadCodec;
impl Encoder<DelegateArgsSaleV1Payload> for DelegateArgsSaleV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsSaleV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsSaleV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsSaleV1Payload> for DelegateArgsSaleV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsSaleV1Payload, Error> {
        Ok(DelegateArgsSaleV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsTransferV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsTransferV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsTransferV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsTransferV1PayloadCodec;
impl Encoder<DelegateArgsTransferV1Payload> for DelegateArgsTransferV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsTransferV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsTransferV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsTransferV1Payload> for DelegateArgsTransferV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsTransferV1Payload, Error> {
        Ok(DelegateArgsTransferV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsDataV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsDataV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsDataV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsDataV1PayloadCodec;
impl Encoder<DelegateArgsDataV1Payload> for DelegateArgsDataV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsDataV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsDataV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsDataV1Payload> for DelegateArgsDataV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsDataV1Payload, Error> {
        Ok(DelegateArgsDataV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsUtilityV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsUtilityV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsUtilityV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsUtilityV1PayloadCodec;
impl Encoder<DelegateArgsUtilityV1Payload> for DelegateArgsUtilityV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsUtilityV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsUtilityV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsUtilityV1Payload> for DelegateArgsUtilityV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsUtilityV1Payload, Error> {
        Ok(DelegateArgsUtilityV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsStakingV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsStakingV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsStakingV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsStakingV1PayloadCodec;
impl Encoder<DelegateArgsStakingV1Payload> for DelegateArgsStakingV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsStakingV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsStakingV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsStakingV1Payload> for DelegateArgsStakingV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsStakingV1Payload, Error> {
        Ok(DelegateArgsStakingV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsStandardV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsStandardV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`DelegateArgsStandardV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsStandardV1PayloadCodec;
impl Encoder<DelegateArgsStandardV1Payload> for DelegateArgsStandardV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsStandardV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsStandardV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsStandardV1Payload> for DelegateArgsStandardV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsStandardV1Payload, Error> {
        Ok(DelegateArgsStandardV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsLockedTransferV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsLockedTransferV1Payload {
    /// The `amount` wire field.
    pub amount: u64,
    /// The `locked_address` wire field.
    pub locked_address: Address,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsLockedTransferV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsLockedTransferV1PayloadCodec;
impl Encoder<DelegateArgsLockedTransferV1Payload> for DelegateArgsLockedTransferV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsLockedTransferV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.locked_address)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsLockedTransferV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.amount, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.locked_address, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsLockedTransferV1Payload> for DelegateArgsLockedTransferV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsLockedTransferV1Payload, Error> {
        Ok(DelegateArgsLockedTransferV1Payload {
            amount: Number::<u64>::LE.read(reader)?,
            locked_address: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsProgrammableConfigV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsProgrammableConfigV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsProgrammableConfigV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsProgrammableConfigV1PayloadCodec;
impl Encoder<DelegateArgsProgrammableConfigV1Payload>
    for DelegateArgsProgrammableConfigV1PayloadCodec
{
    fn encoded_len(&self, value: &DelegateArgsProgrammableConfigV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsProgrammableConfigV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsProgrammableConfigV1Payload>
    for DelegateArgsProgrammableConfigV1PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<DelegateArgsProgrammableConfigV1Payload, Error> {
        Ok(DelegateArgsProgrammableConfigV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsAuthorityItemV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsAuthorityItemV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsAuthorityItemV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsAuthorityItemV1PayloadCodec;
impl Encoder<DelegateArgsAuthorityItemV1Payload> for DelegateArgsAuthorityItemV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsAuthorityItemV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsAuthorityItemV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsAuthorityItemV1Payload> for DelegateArgsAuthorityItemV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsAuthorityItemV1Payload, Error> {
        Ok(DelegateArgsAuthorityItemV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsDataItemV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsDataItemV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsDataItemV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsDataItemV1PayloadCodec;
impl Encoder<DelegateArgsDataItemV1Payload> for DelegateArgsDataItemV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsDataItemV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsDataItemV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsDataItemV1Payload> for DelegateArgsDataItemV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsDataItemV1Payload, Error> {
        Ok(DelegateArgsDataItemV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsCollectionItemV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsCollectionItemV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsCollectionItemV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsCollectionItemV1PayloadCodec;
impl Encoder<DelegateArgsCollectionItemV1Payload> for DelegateArgsCollectionItemV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsCollectionItemV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsCollectionItemV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsCollectionItemV1Payload> for DelegateArgsCollectionItemV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsCollectionItemV1Payload, Error> {
        Ok(DelegateArgsCollectionItemV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsProgrammableConfigItemV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsProgrammableConfigItemV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsProgrammableConfigItemV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsProgrammableConfigItemV1PayloadCodec;
impl Encoder<DelegateArgsProgrammableConfigItemV1Payload>
    for DelegateArgsProgrammableConfigItemV1PayloadCodec
{
    fn encoded_len(
        &self,
        value: &DelegateArgsProgrammableConfigItemV1Payload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsProgrammableConfigItemV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsProgrammableConfigItemV1Payload>
    for DelegateArgsProgrammableConfigItemV1PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<DelegateArgsProgrammableConfigItemV1Payload, Error> {
        Ok(DelegateArgsProgrammableConfigItemV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `DelegateArgsPrintDelegateV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegateArgsPrintDelegateV1Payload {
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`DelegateArgsPrintDelegateV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsPrintDelegateV1PayloadCodec;
impl Encoder<DelegateArgsPrintDelegateV1Payload> for DelegateArgsPrintDelegateV1PayloadCodec {
    fn encoded_len(&self, value: &DelegateArgsPrintDelegateV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &DelegateArgsPrintDelegateV1Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<DelegateArgsPrintDelegateV1Payload> for DelegateArgsPrintDelegateV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgsPrintDelegateV1Payload, Error> {
        Ok(DelegateArgsPrintDelegateV1Payload {
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DelegateArgs {
    /// The `CollectionV1` variant.
    CollectionV1(DelegateArgsCollectionV1Payload),
    /// The `SaleV1` variant.
    SaleV1(DelegateArgsSaleV1Payload),
    /// The `TransferV1` variant.
    TransferV1(DelegateArgsTransferV1Payload),
    /// The `DataV1` variant.
    DataV1(DelegateArgsDataV1Payload),
    /// The `UtilityV1` variant.
    UtilityV1(DelegateArgsUtilityV1Payload),
    /// The `StakingV1` variant.
    StakingV1(DelegateArgsStakingV1Payload),
    /// The `StandardV1` variant.
    StandardV1(DelegateArgsStandardV1Payload),
    /// The `LockedTransferV1` variant.
    LockedTransferV1(DelegateArgsLockedTransferV1Payload),
    /// The `ProgrammableConfigV1` variant.
    ProgrammableConfigV1(DelegateArgsProgrammableConfigV1Payload),
    /// The `AuthorityItemV1` variant.
    AuthorityItemV1(DelegateArgsAuthorityItemV1Payload),
    /// The `DataItemV1` variant.
    DataItemV1(DelegateArgsDataItemV1Payload),
    /// The `CollectionItemV1` variant.
    CollectionItemV1(DelegateArgsCollectionItemV1Payload),
    /// The `ProgrammableConfigItemV1` variant.
    ProgrammableConfigItemV1(DelegateArgsProgrammableConfigItemV1Payload),
    /// The `PrintDelegateV1` variant.
    PrintDelegateV1(DelegateArgsPrintDelegateV1Payload),
}
/// Bounded codec for [`DelegateArgs`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateArgsCodec;
const DELEGATEARGS_TAG_0: u8 = 0;
const DELEGATEARGS_TAG_1: u8 = 1;
const DELEGATEARGS_TAG_2: u8 = 2;
const DELEGATEARGS_TAG_3: u8 = 3;
const DELEGATEARGS_TAG_4: u8 = 4;
const DELEGATEARGS_TAG_5: u8 = 5;
const DELEGATEARGS_TAG_6: u8 = 6;
const DELEGATEARGS_TAG_7: u8 = 7;
const DELEGATEARGS_TAG_8: u8 = 8;
const DELEGATEARGS_TAG_9: u8 = 9;
const DELEGATEARGS_TAG_10: u8 = 10;
const DELEGATEARGS_TAG_11: u8 = 11;
const DELEGATEARGS_TAG_12: u8 = 12;
const DELEGATEARGS_TAG_13: u8 = 13;
impl Encoder<DelegateArgs> for DelegateArgsCodec {
    fn encoded_len(&self, value: &DelegateArgs) -> Result<usize, Error> {
        let size = match value {
            DelegateArgs::CollectionV1(value) => {
                DelegateArgsCollectionV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::SaleV1(value) => DelegateArgsSaleV1PayloadCodec.encoded_len(value)?,
            DelegateArgs::TransferV1(value) => {
                DelegateArgsTransferV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::DataV1(value) => DelegateArgsDataV1PayloadCodec.encoded_len(value)?,
            DelegateArgs::UtilityV1(value) => {
                DelegateArgsUtilityV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::StakingV1(value) => {
                DelegateArgsStakingV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::StandardV1(value) => {
                DelegateArgsStandardV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::LockedTransferV1(value) => {
                DelegateArgsLockedTransferV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::ProgrammableConfigV1(value) => {
                DelegateArgsProgrammableConfigV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::AuthorityItemV1(value) => {
                DelegateArgsAuthorityItemV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::DataItemV1(value) => {
                DelegateArgsDataItemV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::CollectionItemV1(value) => {
                DelegateArgsCollectionItemV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::ProgrammableConfigItemV1(value) => {
                DelegateArgsProgrammableConfigItemV1PayloadCodec.encoded_len(value)?
            }
            DelegateArgs::PrintDelegateV1(value) => {
                DelegateArgsPrintDelegateV1PayloadCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &DelegateArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            DelegateArgs::CollectionV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_0, writer)?;
                DelegateArgsCollectionV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::SaleV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_1, writer)?;
                DelegateArgsSaleV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::TransferV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_2, writer)?;
                DelegateArgsTransferV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::DataV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_3, writer)?;
                DelegateArgsDataV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::UtilityV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_4, writer)?;
                DelegateArgsUtilityV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::StakingV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_5, writer)?;
                DelegateArgsStakingV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::StandardV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_6, writer)?;
                DelegateArgsStandardV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::LockedTransferV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_7, writer)?;
                DelegateArgsLockedTransferV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::ProgrammableConfigV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_8, writer)?;
                DelegateArgsProgrammableConfigV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::AuthorityItemV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_9, writer)?;
                DelegateArgsAuthorityItemV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::DataItemV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_10, writer)?;
                DelegateArgsDataItemV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::CollectionItemV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_11, writer)?;
                DelegateArgsCollectionItemV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::ProgrammableConfigItemV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_12, writer)?;
                DelegateArgsProgrammableConfigItemV1PayloadCodec.write(value, writer)
            }
            DelegateArgs::PrintDelegateV1(value) => {
                Number::<u8>::LE.write(&DELEGATEARGS_TAG_13, writer)?;
                DelegateArgsPrintDelegateV1PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<DelegateArgs> for DelegateArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DelegateArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            DELEGATEARGS_TAG_0 => Ok(DelegateArgs::CollectionV1(
                DelegateArgsCollectionV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_1 => Ok(DelegateArgs::SaleV1(
                DelegateArgsSaleV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_2 => Ok(DelegateArgs::TransferV1(
                DelegateArgsTransferV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_3 => Ok(DelegateArgs::DataV1(
                DelegateArgsDataV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_4 => Ok(DelegateArgs::UtilityV1(
                DelegateArgsUtilityV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_5 => Ok(DelegateArgs::StakingV1(
                DelegateArgsStakingV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_6 => Ok(DelegateArgs::StandardV1(
                DelegateArgsStandardV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_7 => Ok(DelegateArgs::LockedTransferV1(
                DelegateArgsLockedTransferV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_8 => Ok(DelegateArgs::ProgrammableConfigV1(
                DelegateArgsProgrammableConfigV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_9 => Ok(DelegateArgs::AuthorityItemV1(
                DelegateArgsAuthorityItemV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_10 => Ok(DelegateArgs::DataItemV1(
                DelegateArgsDataItemV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_11 => Ok(DelegateArgs::CollectionItemV1(
                DelegateArgsCollectionItemV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_12 => Ok(DelegateArgs::ProgrammableConfigItemV1(
                DelegateArgsProgrammableConfigItemV1PayloadCodec.read(reader)?,
            )),
            DELEGATEARGS_TAG_13 => Ok(DelegateArgs::PrintDelegateV1(
                DelegateArgsPrintDelegateV1PayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `UpdateArgsV1Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsV1Payload {
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
    /// The `data` wire field.
    pub data: Option<Data>,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: Option<bool>,
    /// The `is_mutable` wire field.
    pub is_mutable: Option<bool>,
    /// The `collection` wire field.
    pub collection: CollectionToggle,
    /// The `collection_details` wire field.
    pub collection_details: CollectionDetailsToggle,
    /// The `uses` wire field.
    pub uses: UsesToggle,
    /// The `rule_set` wire field.
    pub rule_set: RuleSetToggle,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsV1Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsV1PayloadCodec;
impl Encoder<UpdateArgsV1Payload> for UpdateArgsV1PayloadCodec {
    fn encoded_len(&self, value: &UpdateArgsV1Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(DataCodec).encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(CollectionToggleCodec.encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(CollectionDetailsToggleCodec.encoded_len(&value.collection_details)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(UsesToggleCodec.encoded_len(&value.uses)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(RuleSetToggleCodec.encoded_len(&value.rule_set)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UpdateArgsV1Payload, writer: &mut Writer<'_>) -> Result<(), Error> {
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_update_authority, writer)?;
        OptionCodec::u8(DataCodec).write(&value.data, writer)?;
        OptionCodec::u8(Bool).write(&value.primary_sale_happened, writer)?;
        OptionCodec::u8(Bool).write(&value.is_mutable, writer)?;
        CollectionToggleCodec.write(&value.collection, writer)?;
        CollectionDetailsToggleCodec.write(&value.collection_details, writer)?;
        UsesToggleCodec.write(&value.uses, writer)?;
        RuleSetToggleCodec.write(&value.rule_set, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsV1Payload> for UpdateArgsV1PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateArgsV1Payload, Error> {
        Ok(UpdateArgsV1Payload {
            new_update_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            data: OptionCodec::u8(DataCodec).read(reader)?,
            primary_sale_happened: OptionCodec::u8(Bool).read(reader)?,
            is_mutable: OptionCodec::u8(Bool).read(reader)?,
            collection: CollectionToggleCodec.read(reader)?,
            collection_details: CollectionDetailsToggleCodec.read(reader)?,
            uses: UsesToggleCodec.read(reader)?,
            rule_set: RuleSetToggleCodec.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsUpdateAuthorityV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsUpdateAuthorityV2Payload {
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
    /// The `data` wire field.
    pub data: Option<Data>,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: Option<bool>,
    /// The `is_mutable` wire field.
    pub is_mutable: Option<bool>,
    /// The `collection` wire field.
    pub collection: CollectionToggle,
    /// The `collection_details` wire field.
    pub collection_details: CollectionDetailsToggle,
    /// The `uses` wire field.
    pub uses: UsesToggle,
    /// The `rule_set` wire field.
    pub rule_set: RuleSetToggle,
    /// The `token_standard` wire field.
    pub token_standard: Option<TokenStandard>,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsUpdateAuthorityV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsUpdateAuthorityV2PayloadCodec;
impl Encoder<UpdateArgsAsUpdateAuthorityV2Payload> for UpdateArgsAsUpdateAuthorityV2PayloadCodec {
    fn encoded_len(&self, value: &UpdateArgsAsUpdateAuthorityV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(DataCodec).encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(CollectionToggleCodec.encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(CollectionDetailsToggleCodec.encoded_len(&value.collection_details)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(UsesToggleCodec.encoded_len(&value.uses)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(RuleSetToggleCodec.encoded_len(&value.rule_set)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(TokenStandardCodec).encoded_len(&value.token_standard)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsUpdateAuthorityV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_update_authority, writer)?;
        OptionCodec::u8(DataCodec).write(&value.data, writer)?;
        OptionCodec::u8(Bool).write(&value.primary_sale_happened, writer)?;
        OptionCodec::u8(Bool).write(&value.is_mutable, writer)?;
        CollectionToggleCodec.write(&value.collection, writer)?;
        CollectionDetailsToggleCodec.write(&value.collection_details, writer)?;
        UsesToggleCodec.write(&value.uses, writer)?;
        RuleSetToggleCodec.write(&value.rule_set, writer)?;
        OptionCodec::u8(TokenStandardCodec).write(&value.token_standard, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsUpdateAuthorityV2Payload> for UpdateArgsAsUpdateAuthorityV2PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateArgsAsUpdateAuthorityV2Payload, Error> {
        Ok(UpdateArgsAsUpdateAuthorityV2Payload {
            new_update_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            data: OptionCodec::u8(DataCodec).read(reader)?,
            primary_sale_happened: OptionCodec::u8(Bool).read(reader)?,
            is_mutable: OptionCodec::u8(Bool).read(reader)?,
            collection: CollectionToggleCodec.read(reader)?,
            collection_details: CollectionDetailsToggleCodec.read(reader)?,
            uses: UsesToggleCodec.read(reader)?,
            rule_set: RuleSetToggleCodec.read(reader)?,
            token_standard: OptionCodec::u8(TokenStandardCodec).read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsAuthorityItemDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsAuthorityItemDelegateV2Payload {
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: Option<bool>,
    /// The `is_mutable` wire field.
    pub is_mutable: Option<bool>,
    /// The `token_standard` wire field.
    pub token_standard: Option<TokenStandard>,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsAuthorityItemDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsAuthorityItemDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsAuthorityItemDelegateV2Payload>
    for UpdateArgsAsAuthorityItemDelegateV2PayloadCodec
{
    fn encoded_len(
        &self,
        value: &UpdateArgsAsAuthorityItemDelegateV2Payload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.primary_sale_happened)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Bool).encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(TokenStandardCodec).encoded_len(&value.token_standard)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsAuthorityItemDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_update_authority, writer)?;
        OptionCodec::u8(Bool).write(&value.primary_sale_happened, writer)?;
        OptionCodec::u8(Bool).write(&value.is_mutable, writer)?;
        OptionCodec::u8(TokenStandardCodec).write(&value.token_standard, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsAuthorityItemDelegateV2Payload>
    for UpdateArgsAsAuthorityItemDelegateV2PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateArgsAsAuthorityItemDelegateV2Payload, Error> {
        Ok(UpdateArgsAsAuthorityItemDelegateV2Payload {
            new_update_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            primary_sale_happened: OptionCodec::u8(Bool).read(reader)?,
            is_mutable: OptionCodec::u8(Bool).read(reader)?,
            token_standard: OptionCodec::u8(TokenStandardCodec).read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsCollectionDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsCollectionDelegateV2Payload {
    /// The `collection` wire field.
    pub collection: CollectionToggle,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsCollectionDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsCollectionDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsCollectionDelegateV2Payload>
    for UpdateArgsAsCollectionDelegateV2PayloadCodec
{
    fn encoded_len(&self, value: &UpdateArgsAsCollectionDelegateV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(CollectionToggleCodec.encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsCollectionDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        CollectionToggleCodec.write(&value.collection, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsCollectionDelegateV2Payload>
    for UpdateArgsAsCollectionDelegateV2PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateArgsAsCollectionDelegateV2Payload, Error> {
        Ok(UpdateArgsAsCollectionDelegateV2Payload {
            collection: CollectionToggleCodec.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsDataDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsDataDelegateV2Payload {
    /// The `data` wire field.
    pub data: Option<Data>,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsDataDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsDataDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsDataDelegateV2Payload> for UpdateArgsAsDataDelegateV2PayloadCodec {
    fn encoded_len(&self, value: &UpdateArgsAsDataDelegateV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(OptionCodec::u8(DataCodec).encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsDataDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(DataCodec).write(&value.data, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsDataDelegateV2Payload> for UpdateArgsAsDataDelegateV2PayloadCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateArgsAsDataDelegateV2Payload, Error> {
        Ok(UpdateArgsAsDataDelegateV2Payload {
            data: OptionCodec::u8(DataCodec).read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsProgrammableConfigDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsProgrammableConfigDelegateV2Payload {
    /// The `rule_set` wire field.
    pub rule_set: RuleSetToggle,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsProgrammableConfigDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsProgrammableConfigDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsProgrammableConfigDelegateV2Payload>
    for UpdateArgsAsProgrammableConfigDelegateV2PayloadCodec
{
    fn encoded_len(
        &self,
        value: &UpdateArgsAsProgrammableConfigDelegateV2Payload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(RuleSetToggleCodec.encoded_len(&value.rule_set)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsProgrammableConfigDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        RuleSetToggleCodec.write(&value.rule_set, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsProgrammableConfigDelegateV2Payload>
    for UpdateArgsAsProgrammableConfigDelegateV2PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateArgsAsProgrammableConfigDelegateV2Payload, Error> {
        Ok(UpdateArgsAsProgrammableConfigDelegateV2Payload {
            rule_set: RuleSetToggleCodec.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsDataItemDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsDataItemDelegateV2Payload {
    /// The `data` wire field.
    pub data: Option<Data>,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsDataItemDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsDataItemDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsDataItemDelegateV2Payload> for UpdateArgsAsDataItemDelegateV2PayloadCodec {
    fn encoded_len(&self, value: &UpdateArgsAsDataItemDelegateV2Payload) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(OptionCodec::u8(DataCodec).encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsDataItemDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        OptionCodec::u8(DataCodec).write(&value.data, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsDataItemDelegateV2Payload> for UpdateArgsAsDataItemDelegateV2PayloadCodec {
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateArgsAsDataItemDelegateV2Payload, Error> {
        Ok(UpdateArgsAsDataItemDelegateV2Payload {
            data: OptionCodec::u8(DataCodec).read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsCollectionItemDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsCollectionItemDelegateV2Payload {
    /// The `collection` wire field.
    pub collection: CollectionToggle,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsCollectionItemDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsCollectionItemDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsCollectionItemDelegateV2Payload>
    for UpdateArgsAsCollectionItemDelegateV2PayloadCodec
{
    fn encoded_len(
        &self,
        value: &UpdateArgsAsCollectionItemDelegateV2Payload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(CollectionToggleCodec.encoded_len(&value.collection)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsCollectionItemDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        CollectionToggleCodec.write(&value.collection, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsCollectionItemDelegateV2Payload>
    for UpdateArgsAsCollectionItemDelegateV2PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateArgsAsCollectionItemDelegateV2Payload, Error> {
        Ok(UpdateArgsAsCollectionItemDelegateV2Payload {
            collection: CollectionToggleCodec.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Validated data for `UpdateArgsAsProgrammableConfigItemDelegateV2Payload`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateArgsAsProgrammableConfigItemDelegateV2Payload {
    /// The `rule_set` wire field.
    pub rule_set: RuleSetToggle,
    /// The `authorization_data` wire field.
    pub authorization_data: Option<AuthorizationData>,
}
/// Bounded codec for [`UpdateArgsAsProgrammableConfigItemDelegateV2Payload`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsAsProgrammableConfigItemDelegateV2PayloadCodec;
impl Encoder<UpdateArgsAsProgrammableConfigItemDelegateV2Payload>
    for UpdateArgsAsProgrammableConfigItemDelegateV2PayloadCodec
{
    fn encoded_len(
        &self,
        value: &UpdateArgsAsProgrammableConfigItemDelegateV2Payload,
    ) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(RuleSetToggleCodec.encoded_len(&value.rule_set)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(AuthorizationDataCodec).encoded_len(&value.authorization_data)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdateArgsAsProgrammableConfigItemDelegateV2Payload,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        RuleSetToggleCodec.write(&value.rule_set, writer)?;
        OptionCodec::u8(AuthorizationDataCodec).write(&value.authorization_data, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateArgsAsProgrammableConfigItemDelegateV2Payload>
    for UpdateArgsAsProgrammableConfigItemDelegateV2PayloadCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<UpdateArgsAsProgrammableConfigItemDelegateV2Payload, Error> {
        Ok(UpdateArgsAsProgrammableConfigItemDelegateV2Payload {
            rule_set: RuleSetToggleCodec.read(reader)?,
            authorization_data: OptionCodec::u8(AuthorizationDataCodec).read(reader)?,
        })
    }
}
/// Tagged data with source-defined payload framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateArgs {
    /// The `V1` variant.
    V1(UpdateArgsV1Payload),
    /// The `AsUpdateAuthorityV2` variant.
    AsUpdateAuthorityV2(UpdateArgsAsUpdateAuthorityV2Payload),
    /// The `AsAuthorityItemDelegateV2` variant.
    AsAuthorityItemDelegateV2(UpdateArgsAsAuthorityItemDelegateV2Payload),
    /// The `AsCollectionDelegateV2` variant.
    AsCollectionDelegateV2(UpdateArgsAsCollectionDelegateV2Payload),
    /// The `AsDataDelegateV2` variant.
    AsDataDelegateV2(UpdateArgsAsDataDelegateV2Payload),
    /// The `AsProgrammableConfigDelegateV2` variant.
    AsProgrammableConfigDelegateV2(UpdateArgsAsProgrammableConfigDelegateV2Payload),
    /// The `AsDataItemDelegateV2` variant.
    AsDataItemDelegateV2(UpdateArgsAsDataItemDelegateV2Payload),
    /// The `AsCollectionItemDelegateV2` variant.
    AsCollectionItemDelegateV2(UpdateArgsAsCollectionItemDelegateV2Payload),
    /// The `AsProgrammableConfigItemDelegateV2` variant.
    AsProgrammableConfigItemDelegateV2(UpdateArgsAsProgrammableConfigItemDelegateV2Payload),
}
/// Bounded codec for [`UpdateArgs`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateArgsCodec;
const UPDATEARGS_TAG_0: u8 = 0;
const UPDATEARGS_TAG_1: u8 = 1;
const UPDATEARGS_TAG_2: u8 = 2;
const UPDATEARGS_TAG_3: u8 = 3;
const UPDATEARGS_TAG_4: u8 = 4;
const UPDATEARGS_TAG_5: u8 = 5;
const UPDATEARGS_TAG_6: u8 = 6;
const UPDATEARGS_TAG_7: u8 = 7;
const UPDATEARGS_TAG_8: u8 = 8;
impl Encoder<UpdateArgs> for UpdateArgsCodec {
    fn encoded_len(&self, value: &UpdateArgs) -> Result<usize, Error> {
        let size = match value {
            UpdateArgs::V1(value) => UpdateArgsV1PayloadCodec.encoded_len(value)?,
            UpdateArgs::AsUpdateAuthorityV2(value) => {
                UpdateArgsAsUpdateAuthorityV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsAuthorityItemDelegateV2(value) => {
                UpdateArgsAsAuthorityItemDelegateV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsCollectionDelegateV2(value) => {
                UpdateArgsAsCollectionDelegateV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsDataDelegateV2(value) => {
                UpdateArgsAsDataDelegateV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsProgrammableConfigDelegateV2(value) => {
                UpdateArgsAsProgrammableConfigDelegateV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsDataItemDelegateV2(value) => {
                UpdateArgsAsDataItemDelegateV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsCollectionItemDelegateV2(value) => {
                UpdateArgsAsCollectionItemDelegateV2PayloadCodec.encoded_len(value)?
            }
            UpdateArgs::AsProgrammableConfigItemDelegateV2(value) => {
                UpdateArgsAsProgrammableConfigItemDelegateV2PayloadCodec.encoded_len(value)?
            }
        };
        size.checked_add(core::mem::size_of::<u8>())
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &UpdateArgs, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UpdateArgs::V1(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_0, writer)?;
                UpdateArgsV1PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsUpdateAuthorityV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_1, writer)?;
                UpdateArgsAsUpdateAuthorityV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsAuthorityItemDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_2, writer)?;
                UpdateArgsAsAuthorityItemDelegateV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsCollectionDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_3, writer)?;
                UpdateArgsAsCollectionDelegateV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsDataDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_4, writer)?;
                UpdateArgsAsDataDelegateV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsProgrammableConfigDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_5, writer)?;
                UpdateArgsAsProgrammableConfigDelegateV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsDataItemDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_6, writer)?;
                UpdateArgsAsDataItemDelegateV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsCollectionItemDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_7, writer)?;
                UpdateArgsAsCollectionItemDelegateV2PayloadCodec.write(value, writer)
            }
            UpdateArgs::AsProgrammableConfigItemDelegateV2(value) => {
                Number::<u8>::LE.write(&UPDATEARGS_TAG_8, writer)?;
                UpdateArgsAsProgrammableConfigItemDelegateV2PayloadCodec.write(value, writer)
            }
        }
    }
}
impl Decoder<UpdateArgs> for UpdateArgsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateArgs, Error> {
        reader.transaction(|reader| match Number::<u8>::LE.read(reader)? {
            UPDATEARGS_TAG_0 => Ok(UpdateArgs::V1(UpdateArgsV1PayloadCodec.read(reader)?)),
            UPDATEARGS_TAG_1 => Ok(UpdateArgs::AsUpdateAuthorityV2(
                UpdateArgsAsUpdateAuthorityV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_2 => Ok(UpdateArgs::AsAuthorityItemDelegateV2(
                UpdateArgsAsAuthorityItemDelegateV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_3 => Ok(UpdateArgs::AsCollectionDelegateV2(
                UpdateArgsAsCollectionDelegateV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_4 => Ok(UpdateArgs::AsDataDelegateV2(
                UpdateArgsAsDataDelegateV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_5 => Ok(UpdateArgs::AsProgrammableConfigDelegateV2(
                UpdateArgsAsProgrammableConfigDelegateV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_6 => Ok(UpdateArgs::AsDataItemDelegateV2(
                UpdateArgsAsDataItemDelegateV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_7 => Ok(UpdateArgs::AsCollectionItemDelegateV2(
                UpdateArgsAsCollectionItemDelegateV2PayloadCodec.read(reader)?,
            )),
            UPDATEARGS_TAG_8 => Ok(UpdateArgs::AsProgrammableConfigItemDelegateV2(
                UpdateArgsAsProgrammableConfigItemDelegateV2PayloadCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}
/// Validated data for `Edition`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edition {
    /// The `parent` wire field.
    pub parent: Address,
    /// The `edition` wire field.
    pub edition: u64,
}
/// Bounded codec for [`Edition`].
#[derive(Clone, Copy, Debug)]
pub struct EditionCodec;
impl Encoder<Edition> for EditionCodec {
    fn encoded_len(&self, value: &Edition) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[1],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.parent)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.edition)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Edition, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[1];
        let _record_start = writer.position();
        Affix {
            prefix: &[1],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.parent, writer)?;
        Number::<u64>::LE.write(&value.edition, writer)?;
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
impl Decoder<Edition> for EditionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Edition, Error> {
        const _DISCRIMINATOR: &[u8] = &[1];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Edition {
            parent: Affix {
                prefix: &[1],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            edition: Number::<u64>::LE.read(reader)?,
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
/// Validated data for `MasterEditionV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MasterEditionV1 {
    /// The `supply` wire field.
    pub supply: u64,
    /// The `max_supply` wire field.
    pub max_supply: Option<u64>,
    /// The `printing_mint` wire field.
    pub printing_mint: Address,
    /// The `one_time_printing_authorization_mint` wire field.
    pub one_time_printing_authorization_mint: Address,
}
/// Bounded codec for [`MasterEditionV1`].
#[derive(Clone, Copy, Debug)]
pub struct MasterEditionV1Codec;
impl Encoder<MasterEditionV1> for MasterEditionV1Codec {
    fn encoded_len(&self, value: &MasterEditionV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[2],
                    item: Number::<u64>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.supply)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.max_supply)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.printing_mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.one_time_printing_authorization_mint)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MasterEditionV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[2];
        let _record_start = writer.position();
        Affix {
            prefix: &[2],
            item: Number::<u64>::LE,
            suffix: &[],
        }
        .write(&value.supply, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.max_supply, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.printing_mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.one_time_printing_authorization_mint, writer)?;
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
impl Decoder<MasterEditionV1> for MasterEditionV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MasterEditionV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[2];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = MasterEditionV1 {
            supply: Affix {
                prefix: &[2],
                item: Number::<u64>::LE,
                suffix: &[],
            }
            .read(reader)?,
            max_supply: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
            printing_mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            one_time_printing_authorization_mint: Mapped::new(
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
/// Validated data for `ReservationListV1`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservationListV1 {
    /// The `master_edition` wire field.
    pub master_edition: Address,
    /// The `supply_snapshot` wire field.
    pub supply_snapshot: Option<u64>,
    /// The `reservations` wire field.
    pub reservations: Vec<ReservationV1>,
}
/// Bounded codec for [`ReservationListV1`].
#[derive(Clone, Copy, Debug)]
pub struct ReservationListV1Codec;
impl Encoder<ReservationListV1> for ReservationListV1Codec {
    fn encoded_len(&self, value: &ReservationListV1) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[3],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.master_edition)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.supply_snapshot)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ReservationV1Codec,
                    minimum: 34,
                }
                .encoded_len(value.reservations.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ReservationListV1, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[3];
        let _record_start = writer.position();
        Affix {
            prefix: &[3],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.master_edition, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.supply_snapshot, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ReservationV1Codec,
            minimum: 34,
        }
        .write(value.reservations.as_slice(), writer)?;
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
impl Decoder<ReservationListV1> for ReservationListV1Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ReservationListV1, Error> {
        const _DISCRIMINATOR: &[u8] = &[3];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = ReservationListV1 {
            master_edition: Affix {
                prefix: &[3],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            supply_snapshot: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
            reservations: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ReservationV1Codec,
                minimum: 34,
            }
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
/// Validated data for `Metadata`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    /// The `update_authority` wire field.
    pub update_authority: Address,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `data` wire field.
    pub data: Data,
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
    /// The `collection_details` wire field.
    pub collection_details: Option<CollectionDetails>,
    /// The `programmable_config` wire field.
    pub programmable_config: Option<ProgrammableConfig>,
}
/// Bounded codec for [`Metadata`].
#[derive(Clone, Copy, Debug)]
pub struct MetadataCodec;
impl Encoder<Metadata> for MetadataCodec {
    fn encoded_len(&self, value: &Metadata) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[4],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(DataCodec.encoded_len(&value.data)?)
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
            .checked_add(
                OptionCodec::u8(CollectionDetailsCodec).encoded_len(&value.collection_details)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(ProgrammableConfigCodec).encoded_len(&value.programmable_config)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Metadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[4];
        let _record_start = writer.position();
        Affix {
            prefix: &[4],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.update_authority, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        DataCodec.write(&value.data, writer)?;
        Bool.write(&value.primary_sale_happened, writer)?;
        Bool.write(&value.is_mutable, writer)?;
        OptionCodec::u8(Number::<u8>::LE).write(&value.edition_nonce, writer)?;
        OptionCodec::u8(TokenStandardCodec).write(&value.token_standard, writer)?;
        OptionCodec::u8(CollectionCodec).write(&value.collection, writer)?;
        OptionCodec::u8(UsesCodec).write(&value.uses, writer)?;
        OptionCodec::u8(CollectionDetailsCodec).write(&value.collection_details, writer)?;
        OptionCodec::u8(ProgrammableConfigCodec).write(&value.programmable_config, writer)?;
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
impl Decoder<Metadata> for MetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Metadata, Error> {
        const _DISCRIMINATOR: &[u8] = &[4];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = Metadata {
            update_authority: Affix {
                prefix: &[4],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            mint: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )
            .read(reader)?,
            data: DataCodec.read(reader)?,
            primary_sale_happened: Bool.read(reader)?,
            is_mutable: Bool.read(reader)?,
            edition_nonce: OptionCodec::u8(Number::<u8>::LE).read(reader)?,
            token_standard: OptionCodec::u8(TokenStandardCodec).read(reader)?,
            collection: OptionCodec::u8(CollectionCodec).read(reader)?,
            uses: OptionCodec::u8(UsesCodec).read(reader)?,
            collection_details: OptionCodec::u8(CollectionDetailsCodec).read(reader)?,
            programmable_config: OptionCodec::u8(ProgrammableConfigCodec).read(reader)?,
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
/// Validated data for `ReservationListV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservationListV2 {
    /// The `master_edition` wire field.
    pub master_edition: Address,
    /// The `supply_snapshot` wire field.
    pub supply_snapshot: Option<u64>,
    /// The `reservations` wire field.
    pub reservations: Vec<Reservation>,
    /// The `total_reservation_spots` wire field.
    pub total_reservation_spots: u64,
    /// The `current_reservation_spots` wire field.
    pub current_reservation_spots: u64,
}
/// Bounded codec for [`ReservationListV2`].
#[derive(Clone, Copy, Debug)]
pub struct ReservationListV2Codec;
impl Encoder<ReservationListV2> for ReservationListV2Codec {
    fn encoded_len(&self, value: &ReservationListV2) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[5],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.master_edition)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.supply_snapshot)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: ReservationCodec,
                    minimum: 48,
                }
                .encoded_len(value.reservations.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.total_reservation_spots)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.current_reservation_spots)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ReservationListV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[5];
        let _record_start = writer.position();
        Affix {
            prefix: &[5],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.master_edition, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.supply_snapshot, writer)?;
        CheckedCounted {
            prefix: Number::<u32>::LE,
            item: ReservationCodec,
            minimum: 48,
        }
        .write(value.reservations.as_slice(), writer)?;
        Number::<u64>::LE.write(&value.total_reservation_spots, writer)?;
        Number::<u64>::LE.write(&value.current_reservation_spots, writer)?;
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
impl Decoder<ReservationListV2> for ReservationListV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ReservationListV2, Error> {
        const _DISCRIMINATOR: &[u8] = &[5];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = ReservationListV2 {
            master_edition: Affix {
                prefix: &[5],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            supply_snapshot: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
            reservations: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: ReservationCodec,
                minimum: 48,
            }
            .read(reader)?,
            total_reservation_spots: Number::<u64>::LE.read(reader)?,
            current_reservation_spots: Number::<u64>::LE.read(reader)?,
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
/// Validated data for `MasterEditionV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MasterEditionV2 {
    /// The `supply` wire field.
    pub supply: u64,
    /// The `max_supply` wire field.
    pub max_supply: Option<u64>,
}
/// Bounded codec for [`MasterEditionV2`].
#[derive(Clone, Copy, Debug)]
pub struct MasterEditionV2Codec;
impl Encoder<MasterEditionV2> for MasterEditionV2Codec {
    fn encoded_len(&self, value: &MasterEditionV2) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[6],
                    item: Number::<u64>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.supply)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.max_supply)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MasterEditionV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[6];
        let _record_start = writer.position();
        Affix {
            prefix: &[6],
            item: Number::<u64>::LE,
            suffix: &[],
        }
        .write(&value.supply, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.max_supply, writer)?;
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
impl Decoder<MasterEditionV2> for MasterEditionV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MasterEditionV2, Error> {
        const _DISCRIMINATOR: &[u8] = &[6];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = MasterEditionV2 {
            supply: Affix {
                prefix: &[6],
                item: Number::<u64>::LE,
                suffix: &[],
            }
            .read(reader)?,
            max_supply: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
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
/// Validated data for `EditionMarker`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditionMarker {
    /// The `ledger` wire field.
    pub ledger: [u8; 31],
}
/// Bounded codec for [`EditionMarker`].
#[derive(Clone, Copy, Debug)]
pub struct EditionMarkerCodec;
impl Encoder<EditionMarker> for EditionMarkerCodec {
    fn encoded_len(&self, value: &EditionMarker) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[7],
                    item: FixedBytes::<31>,
                    suffix: &[],
                }
                .encoded_len(&value.ledger)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &EditionMarker, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[7];
        let _record_start = writer.position();
        Affix {
            prefix: &[7],
            item: FixedBytes::<31>,
            suffix: &[],
        }
        .write(&value.ledger, writer)?;
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
impl Decoder<EditionMarker> for EditionMarkerCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EditionMarker, Error> {
        const _DISCRIMINATOR: &[u8] = &[7];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = EditionMarker {
            ledger: Affix {
                prefix: &[7],
                item: FixedBytes::<31>,
                suffix: &[],
            }
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
/// Validated data for `UseAuthorityRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UseAuthorityRecord {
    /// The `allowed_uses` wire field.
    pub allowed_uses: u64,
    /// The `bump` wire field.
    pub bump: u8,
}
/// Bounded codec for [`UseAuthorityRecord`].
#[derive(Clone, Copy, Debug)]
pub struct UseAuthorityRecordCodec;
impl Encoder<UseAuthorityRecord> for UseAuthorityRecordCodec {
    fn encoded_len(&self, value: &UseAuthorityRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[8],
                    item: Number::<u64>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.allowed_uses)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &UseAuthorityRecord, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[8];
        let _record_start = writer.position();
        Affix {
            prefix: &[8],
            item: Number::<u64>::LE,
            suffix: &[],
        }
        .write(&value.allowed_uses, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
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
impl Decoder<UseAuthorityRecord> for UseAuthorityRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UseAuthorityRecord, Error> {
        const _DISCRIMINATOR: &[u8] = &[8];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = UseAuthorityRecord {
            allowed_uses: Affix {
                prefix: &[8],
                item: Number::<u64>::LE,
                suffix: &[],
            }
            .read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
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
/// Validated data for `CollectionAuthorityRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionAuthorityRecord {
    /// The `bump` wire field.
    pub bump: u8,
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
}
/// Bounded codec for [`CollectionAuthorityRecord`].
#[derive(Clone, Copy, Debug)]
pub struct CollectionAuthorityRecordCodec;
impl Encoder<CollectionAuthorityRecord> for CollectionAuthorityRecordCodec {
    fn encoded_len(&self, value: &CollectionAuthorityRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[9],
                    item: Number::<u8>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.bump)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &CollectionAuthorityRecord,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[9];
        let _record_start = writer.position();
        Affix {
            prefix: &[9],
            item: Number::<u8>::LE,
            suffix: &[],
        }
        .write(&value.bump, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_update_authority, writer)?;
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
impl Decoder<CollectionAuthorityRecord> for CollectionAuthorityRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CollectionAuthorityRecord, Error> {
        const _DISCRIMINATOR: &[u8] = &[9];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = CollectionAuthorityRecord {
            bump: Affix {
                prefix: &[9],
                item: Number::<u8>::LE,
                suffix: &[],
            }
            .read(reader)?,
            new_update_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
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
/// Validated data for `TokenOwnedEscrow`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenOwnedEscrow {
    /// The `base_token` wire field.
    pub base_token: Address,
    /// The `authority` wire field.
    pub authority: EscrowAuthority,
    /// The `bump` wire field.
    pub bump: u8,
}
/// Bounded codec for [`TokenOwnedEscrow`].
#[derive(Clone, Copy, Debug)]
pub struct TokenOwnedEscrowCodec;
impl Encoder<TokenOwnedEscrow> for TokenOwnedEscrowCodec {
    fn encoded_len(&self, value: &TokenOwnedEscrow) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[10],
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    suffix: &[],
                }
                .encoded_len(&value.base_token)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(EscrowAuthorityCodec.encoded_len(&value.authority)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TokenOwnedEscrow, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[10];
        let _record_start = writer.position();
        Affix {
            prefix: &[10],
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            suffix: &[],
        }
        .write(&value.base_token, writer)?;
        EscrowAuthorityCodec.write(&value.authority, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
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
impl Decoder<TokenOwnedEscrow> for TokenOwnedEscrowCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenOwnedEscrow, Error> {
        const _DISCRIMINATOR: &[u8] = &[10];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = TokenOwnedEscrow {
            base_token: Affix {
                prefix: &[10],
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                suffix: &[],
            }
            .read(reader)?,
            authority: EscrowAuthorityCodec.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
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
/// Validated data for `TokenRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenRecord {
    /// The `bump` wire field.
    pub bump: u8,
    /// The `state` wire field.
    pub state: TokenState,
    /// The `rule_set_revision` wire field.
    pub rule_set_revision: Option<u64>,
    /// The `delegate` wire field.
    pub delegate: Option<Address>,
    /// The `delegate_role` wire field.
    pub delegate_role: Option<TokenDelegateRole>,
    /// The `locked_transfer` wire field.
    pub locked_transfer: Option<Address>,
}
/// Bounded codec for [`TokenRecord`].
#[derive(Clone, Copy, Debug)]
pub struct TokenRecordCodec;
impl Encoder<TokenRecord> for TokenRecordCodec {
    fn encoded_len(&self, value: &TokenRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[11],
                    item: Number::<u8>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.bump)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(TokenStateCodec.encoded_len(&value.state)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.rule_set_revision)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.delegate)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(OptionCodec::u8(TokenDelegateRoleCodec).encoded_len(&value.delegate_role)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.locked_transfer)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TokenRecord, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[11];
        let _record_start = writer.position();
        Affix {
            prefix: &[11],
            item: Number::<u8>::LE,
            suffix: &[],
        }
        .write(&value.bump, writer)?;
        TokenStateCodec.write(&value.state, writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.rule_set_revision, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.delegate, writer)?;
        OptionCodec::u8(TokenDelegateRoleCodec).write(&value.delegate_role, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.locked_transfer, writer)?;
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
impl Decoder<TokenRecord> for TokenRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TokenRecord, Error> {
        const _DISCRIMINATOR: &[u8] = &[11];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = TokenRecord {
            bump: Affix {
                prefix: &[11],
                item: Number::<u8>::LE,
                suffix: &[],
            }
            .read(reader)?,
            state: TokenStateCodec.read(reader)?,
            rule_set_revision: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
            delegate: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            delegate_role: OptionCodec::u8(TokenDelegateRoleCodec).read(reader)?,
            locked_transfer: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
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
/// Validated data for `MetadataDelegateRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataDelegateRecord {
    /// The `bump` wire field.
    pub bump: u8,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `delegate` wire field.
    pub delegate: Address,
    /// The `update_authority` wire field.
    pub update_authority: Address,
}
/// Bounded codec for [`MetadataDelegateRecord`].
#[derive(Clone, Copy, Debug)]
pub struct MetadataDelegateRecordCodec;
impl Encoder<MetadataDelegateRecord> for MetadataDelegateRecordCodec {
    fn encoded_len(&self, value: &MetadataDelegateRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[12],
                    item: Number::<u8>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.bump)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
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
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &MetadataDelegateRecord, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[12];
        let _record_start = writer.position();
        Affix {
            prefix: &[12],
            item: Number::<u8>::LE,
            suffix: &[],
        }
        .write(&value.bump, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegate, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.update_authority, writer)?;
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
impl Decoder<MetadataDelegateRecord> for MetadataDelegateRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MetadataDelegateRecord, Error> {
        const _DISCRIMINATOR: &[u8] = &[12];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = MetadataDelegateRecord {
            bump: Affix {
                prefix: &[12],
                item: Number::<u8>::LE,
                suffix: &[],
            }
            .read(reader)?,
            mint: Mapped::new(
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
            update_authority: Mapped::new(
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
/// Validated data for `EditionMarkerV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditionMarkerV2 {
    /// The `ledger` wire field.
    pub ledger: Vec<u8>,
}
/// Bounded codec for [`EditionMarkerV2`].
#[derive(Clone, Copy, Debug)]
pub struct EditionMarkerV2Codec;
impl Encoder<EditionMarkerV2> for EditionMarkerV2Codec {
    fn encoded_len(&self, value: &EditionMarkerV2) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[13],
                    item: CheckedCounted {
                        prefix: Number::<u32>::LE,
                        item: Number::<u8>::LE,
                        minimum: 1,
                    },
                    suffix: &[],
                }
                .encoded_len(value.ledger.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &EditionMarkerV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[13];
        let _record_start = writer.position();
        Affix {
            prefix: &[13],
            item: CheckedCounted {
                prefix: Number::<u32>::LE,
                item: Number::<u8>::LE,
                minimum: 1,
            },
            suffix: &[],
        }
        .write(value.ledger.as_slice(), writer)?;
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
impl Decoder<EditionMarkerV2> for EditionMarkerV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<EditionMarkerV2, Error> {
        const _DISCRIMINATOR: &[u8] = &[13];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = EditionMarkerV2 {
            ledger: Affix {
                prefix: &[13],
                item: CheckedCounted {
                    prefix: Number::<u32>::LE,
                    item: Number::<u8>::LE,
                    minimum: 1,
                },
                suffix: &[],
            }
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
/// Validated data for `HolderDelegateRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HolderDelegateRecord {
    /// The `bump` wire field.
    pub bump: u8,
    /// The `mint` wire field.
    pub mint: Address,
    /// The `delegate` wire field.
    pub delegate: Address,
    /// The `update_authority` wire field.
    pub update_authority: Address,
}
/// Bounded codec for [`HolderDelegateRecord`].
#[derive(Clone, Copy, Debug)]
pub struct HolderDelegateRecordCodec;
impl Encoder<HolderDelegateRecord> for HolderDelegateRecordCodec {
    fn encoded_len(&self, value: &HolderDelegateRecord) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                Affix {
                    prefix: &[14],
                    item: Number::<u8>::LE,
                    suffix: &[],
                }
                .encoded_len(&value.bump)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.mint)?,
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
            .checked_add(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )
                .encoded_len(&value.update_authority)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &HolderDelegateRecord, writer: &mut Writer<'_>) -> Result<(), Error> {
        const _DISCRIMINATOR: &[u8] = &[14];
        let _record_start = writer.position();
        Affix {
            prefix: &[14],
            item: Number::<u8>::LE,
            suffix: &[],
        }
        .write(&value.bump, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.mint, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.delegate, writer)?;
        Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        )
        .write(&value.update_authority, writer)?;
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
impl Decoder<HolderDelegateRecord> for HolderDelegateRecordCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<HolderDelegateRecord, Error> {
        const _DISCRIMINATOR: &[u8] = &[14];
        let _record_start = reader.position();
        let _actual_discriminator = reader
            .remaining()
            .get(.._DISCRIMINATOR.len())
            .ok_or(Error::Truncated)?;
        if _actual_discriminator != _DISCRIMINATOR {
            return Err(Error::InvalidDiscriminator);
        }
        let _record_value = HolderDelegateRecord {
            bump: Affix {
                prefix: &[14],
                item: Number::<u8>::LE,
                suffix: &[],
            }
            .read(reader)?,
            mint: Mapped::new(
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
            update_authority: Mapped::new(
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
/// Validated data for `CreateMetadataAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateMetadataAccount {}
/// Bounded codec for [`CreateMetadataAccount`].
#[derive(Clone, Copy, Debug)]
pub struct CreateMetadataAccountCodec;
impl Encoder<CreateMetadataAccount> for CreateMetadataAccountCodec {
    fn encoded_len(&self, value: &CreateMetadataAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CreateMetadataAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CreateMetadataAccount> for CreateMetadataAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateMetadataAccount, Error> {
        Constant(&[INSTRUCTION_TAG_0]).read(reader)?;
        let _ = reader;
        Ok(CreateMetadataAccount {})
    }
}
impl CreateMetadataAccount {
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
                /* metadata */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = CreateMetadataAccountCodec.encode(self, limits.max_bytes)?;
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
        CreateMetadataAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateMetadataAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateMetadataAccount {}
/// Bounded codec for [`UpdateMetadataAccount`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateMetadataAccountCodec;
impl Encoder<UpdateMetadataAccount> for UpdateMetadataAccountCodec {
    fn encoded_len(&self, value: &UpdateMetadataAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &UpdateMetadataAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_1]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<UpdateMetadataAccount> for UpdateMetadataAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateMetadataAccount, Error> {
        Constant(&[INSTRUCTION_TAG_1]).read(reader)?;
        let _ = reader;
        Ok(UpdateMetadataAccount {})
    }
}
impl UpdateMetadataAccount {
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
                /* metadata */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
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
        let data = UpdateMetadataAccountCodec.encode(self, limits.max_bytes)?;
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
        UpdateMetadataAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeprecatedCreateMasterEdition`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeprecatedCreateMasterEdition {}
/// Bounded codec for [`DeprecatedCreateMasterEdition`].
#[derive(Clone, Copy, Debug)]
pub struct DeprecatedCreateMasterEditionCodec;
impl Encoder<DeprecatedCreateMasterEdition> for DeprecatedCreateMasterEditionCodec {
    fn encoded_len(&self, value: &DeprecatedCreateMasterEdition) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DeprecatedCreateMasterEdition,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_2]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeprecatedCreateMasterEdition> for DeprecatedCreateMasterEditionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeprecatedCreateMasterEdition, Error> {
        Constant(&[INSTRUCTION_TAG_2]).read(reader)?;
        let _ = reader;
        Ok(DeprecatedCreateMasterEdition {})
    }
}
impl DeprecatedCreateMasterEdition {
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
                /* edition */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* printing_mint */ (true, Some(false), false),
                /* one_time_printing_authorization_mint */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
                /* printing_mint_authority */ (false, Some(true), false),
                /* mint_authority */ (false, Some(true), false),
                /* metadata */ (false, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
                /* one_time_printing_authorization_mint_authority */
                (false, Some(true), false),
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
        let data = DeprecatedCreateMasterEditionCodec.encode(self, limits.max_bytes)?;
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
        DeprecatedCreateMasterEditionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeprecatedMintNewEditionFromMasterEditionViaPrintingToken`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {}
/// Bounded codec for [`DeprecatedMintNewEditionFromMasterEditionViaPrintingToken`].
#[derive(Clone, Copy, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec;
impl Encoder<DeprecatedMintNewEditionFromMasterEditionViaPrintingToken>
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec
{
    fn encoded_len(
        &self,
        value: &DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
    ) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_3]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeprecatedMintNewEditionFromMasterEditionViaPrintingToken>
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<DeprecatedMintNewEditionFromMasterEditionViaPrintingToken, Error> {
        Constant(&[INSTRUCTION_TAG_3]).read(reader)?;
        let _ = reader;
        Ok(DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {})
    }
}
impl DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 15 || accounts.len() > 16 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* mint_authority */ (false, Some(true), false),
                /* printing_mint */ (true, Some(false), false),
                /* master_token_account */ (true, Some(false), false),
                /* edition_marker */ (true, Some(false), false),
                /* burn_authority */ (false, Some(true), false),
                /* payer */ (false, Some(true), false),
                /* master_update_authority */ (false, Some(false), false),
                /* master_metadata */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
                /* reservation_list */ (true, Some(false), false),
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
        let data = DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec
            .encode(self, limits.max_bytes)?;
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
        DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec
            .decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdatePrimarySaleHappenedViaToken`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePrimarySaleHappenedViaToken {}
/// Bounded codec for [`UpdatePrimarySaleHappenedViaToken`].
#[derive(Clone, Copy, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenCodec;
impl Encoder<UpdatePrimarySaleHappenedViaToken> for UpdatePrimarySaleHappenedViaTokenCodec {
    fn encoded_len(&self, value: &UpdatePrimarySaleHappenedViaToken) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &UpdatePrimarySaleHappenedViaToken,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_4]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<UpdatePrimarySaleHappenedViaToken> for UpdatePrimarySaleHappenedViaTokenCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdatePrimarySaleHappenedViaToken, Error> {
        Constant(&[INSTRUCTION_TAG_4]).read(reader)?;
        let _ = reader;
        Ok(UpdatePrimarySaleHappenedViaToken {})
    }
}
impl UpdatePrimarySaleHappenedViaToken {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* owner */ (false, Some(true), false),
                /* token */ (false, Some(false), false),
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
        let data = UpdatePrimarySaleHappenedViaTokenCodec.encode(self, limits.max_bytes)?;
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
        UpdatePrimarySaleHappenedViaTokenCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeprecatedSetReservationList`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeprecatedSetReservationList {}
/// Bounded codec for [`DeprecatedSetReservationList`].
#[derive(Clone, Copy, Debug)]
pub struct DeprecatedSetReservationListCodec;
impl Encoder<DeprecatedSetReservationList> for DeprecatedSetReservationListCodec {
    fn encoded_len(&self, value: &DeprecatedSetReservationList) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DeprecatedSetReservationList,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_5]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeprecatedSetReservationList> for DeprecatedSetReservationListCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeprecatedSetReservationList, Error> {
        Constant(&[INSTRUCTION_TAG_5]).read(reader)?;
        let _ = reader;
        Ok(DeprecatedSetReservationList {})
    }
}
impl DeprecatedSetReservationList {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* master_edition */ (true, Some(false), false),
                /* reservation_list */ (true, Some(false), false),
                /* resource */ (false, Some(true), false),
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
        let data = DeprecatedSetReservationListCodec.encode(self, limits.max_bytes)?;
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
        DeprecatedSetReservationListCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeprecatedCreateReservationList`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeprecatedCreateReservationList {}
/// Bounded codec for [`DeprecatedCreateReservationList`].
#[derive(Clone, Copy, Debug)]
pub struct DeprecatedCreateReservationListCodec;
impl Encoder<DeprecatedCreateReservationList> for DeprecatedCreateReservationListCodec {
    fn encoded_len(&self, value: &DeprecatedCreateReservationList) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DeprecatedCreateReservationList,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_6]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeprecatedCreateReservationList> for DeprecatedCreateReservationListCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeprecatedCreateReservationList, Error> {
        Constant(&[INSTRUCTION_TAG_6]).read(reader)?;
        let _ = reader;
        Ok(DeprecatedCreateReservationList {})
    }
}
impl DeprecatedCreateReservationList {
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
                /* reservation_list */ (true, Some(false), false),
                /* payer */ (false, Some(true), false),
                /* update_authority */ (false, Some(true), false),
                /* master_edition */ (false, Some(false), false),
                /* resource */ (false, Some(false), false),
                /* metadata */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = DeprecatedCreateReservationListCodec.encode(self, limits.max_bytes)?;
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
        DeprecatedCreateReservationListCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SignMetadata`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignMetadata {}
/// Bounded codec for [`SignMetadata`].
#[derive(Clone, Copy, Debug)]
pub struct SignMetadataCodec;
impl Encoder<SignMetadata> for SignMetadataCodec {
    fn encoded_len(&self, value: &SignMetadata) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SignMetadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_7]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SignMetadata> for SignMetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SignMetadata, Error> {
        Constant(&[INSTRUCTION_TAG_7]).read(reader)?;
        let _ = reader;
        Ok(SignMetadata {})
    }
}
impl SignMetadata {
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
                /* metadata */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
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
        let data = SignMetadataCodec.encode(self, limits.max_bytes)?;
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
        SignMetadataCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeprecatedMintPrintingTokensViaToken`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeprecatedMintPrintingTokensViaToken {}
/// Bounded codec for [`DeprecatedMintPrintingTokensViaToken`].
#[derive(Clone, Copy, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenCodec;
impl Encoder<DeprecatedMintPrintingTokensViaToken> for DeprecatedMintPrintingTokensViaTokenCodec {
    fn encoded_len(&self, value: &DeprecatedMintPrintingTokensViaToken) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DeprecatedMintPrintingTokensViaToken,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_8]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeprecatedMintPrintingTokensViaToken> for DeprecatedMintPrintingTokensViaTokenCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeprecatedMintPrintingTokensViaToken, Error> {
        Constant(&[INSTRUCTION_TAG_8]).read(reader)?;
        let _ = reader;
        Ok(DeprecatedMintPrintingTokensViaToken {})
    }
}
impl DeprecatedMintPrintingTokensViaToken {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* destination */ (true, Some(false), false),
                /* token */ (true, Some(false), false),
                /* one_time_printing_authorization_mint */ (true, Some(false), false),
                /* printing_mint */ (true, Some(false), false),
                /* burn_authority */ (false, Some(true), false),
                /* metadata */ (false, Some(false), false),
                /* master_edition */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = DeprecatedMintPrintingTokensViaTokenCodec.encode(self, limits.max_bytes)?;
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
        DeprecatedMintPrintingTokensViaTokenCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `DeprecatedMintPrintingTokens`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeprecatedMintPrintingTokens {}
/// Bounded codec for [`DeprecatedMintPrintingTokens`].
#[derive(Clone, Copy, Debug)]
pub struct DeprecatedMintPrintingTokensCodec;
impl Encoder<DeprecatedMintPrintingTokens> for DeprecatedMintPrintingTokensCodec {
    fn encoded_len(&self, value: &DeprecatedMintPrintingTokens) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &DeprecatedMintPrintingTokens,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_9]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeprecatedMintPrintingTokens> for DeprecatedMintPrintingTokensCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeprecatedMintPrintingTokens, Error> {
        Constant(&[INSTRUCTION_TAG_9]).read(reader)?;
        let _ = reader;
        Ok(DeprecatedMintPrintingTokens {})
    }
}
impl DeprecatedMintPrintingTokens {
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
                /* destination */ (true, Some(false), false),
                /* printing_mint */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
                /* metadata */ (false, Some(false), false),
                /* master_edition */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = DeprecatedMintPrintingTokensCodec.encode(self, limits.max_bytes)?;
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
        DeprecatedMintPrintingTokensCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateMasterEdition`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateMasterEdition {}
/// Bounded codec for [`CreateMasterEdition`].
#[derive(Clone, Copy, Debug)]
pub struct CreateMasterEditionCodec;
impl Encoder<CreateMasterEdition> for CreateMasterEditionCodec {
    fn encoded_len(&self, value: &CreateMasterEdition) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CreateMasterEdition, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_10]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CreateMasterEdition> for CreateMasterEditionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateMasterEdition, Error> {
        Constant(&[INSTRUCTION_TAG_10]).read(reader)?;
        let _ = reader;
        Ok(CreateMasterEdition {})
    }
}
impl CreateMasterEdition {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* edition */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
                /* mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* metadata */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = CreateMasterEditionCodec.encode(self, limits.max_bytes)?;
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
        CreateMasterEditionCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintNewEditionFromMasterEditionViaToken`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintNewEditionFromMasterEditionViaToken {
    /// The `mint_new_edition_from_master_edition_via_token_args` wire field.
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
/// Bounded codec for [`MintNewEditionFromMasterEditionViaToken`].
#[derive(Clone, Copy, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenCodec;
impl Encoder<MintNewEditionFromMasterEditionViaToken>
    for MintNewEditionFromMasterEditionViaTokenCodec
{
    fn encoded_len(&self, value: &MintNewEditionFromMasterEditionViaToken) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                MintNewEditionFromMasterEditionViaTokenArgsCodec
                    .encoded_len(&value.mint_new_edition_from_master_edition_via_token_args)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MintNewEditionFromMasterEditionViaToken,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_11]).write(&(), writer)?;
        MintNewEditionFromMasterEditionViaTokenArgsCodec.write(
            &value.mint_new_edition_from_master_edition_via_token_args,
            writer,
        )?;
        Ok(())
    }
}
impl Decoder<MintNewEditionFromMasterEditionViaToken>
    for MintNewEditionFromMasterEditionViaTokenCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<MintNewEditionFromMasterEditionViaToken, Error> {
        Constant(&[INSTRUCTION_TAG_11]).read(reader)?;
        Ok(MintNewEditionFromMasterEditionViaToken {
            mint_new_edition_from_master_edition_via_token_args:
                MintNewEditionFromMasterEditionViaTokenArgsCodec.read(reader)?,
        })
    }
}
impl MintNewEditionFromMasterEditionViaToken {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 13 || accounts.len() > 14 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* new_metadata */ (true, Some(false), false),
                /* new_edition */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), false),
                /* new_mint */ (true, Some(false), false),
                /* edition_mark_pda */ (true, Some(false), false),
                /* new_mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* token_account_owner */ (false, Some(true), false),
                /* token_account */ (false, Some(false), false),
                /* new_metadata_update_authority */ (false, Some(false), false),
                /* metadata */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = MintNewEditionFromMasterEditionViaTokenCodec.encode(self, limits.max_bytes)?;
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
        MintNewEditionFromMasterEditionViaTokenCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ConvertMasterEditionV1ToV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConvertMasterEditionV1ToV2 {}
/// Bounded codec for [`ConvertMasterEditionV1ToV2`].
#[derive(Clone, Copy, Debug)]
pub struct ConvertMasterEditionV1ToV2Codec;
impl Encoder<ConvertMasterEditionV1ToV2> for ConvertMasterEditionV1ToV2Codec {
    fn encoded_len(&self, value: &ConvertMasterEditionV1ToV2) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ConvertMasterEditionV1ToV2,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_12]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ConvertMasterEditionV1ToV2> for ConvertMasterEditionV1ToV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConvertMasterEditionV1ToV2, Error> {
        Constant(&[INSTRUCTION_TAG_12]).read(reader)?;
        let _ = reader;
        Ok(ConvertMasterEditionV1ToV2 {})
    }
}
impl ConvertMasterEditionV1ToV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 3 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* master_edition */ (true, Some(false), false),
                /* one_time_auth */ (true, Some(false), false),
                /* printing_mint */ (true, Some(false), false),
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
        let data = ConvertMasterEditionV1ToV2Codec.encode(self, limits.max_bytes)?;
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
        ConvertMasterEditionV1ToV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `MintNewEditionFromMasterEditionViaVaultProxy`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintNewEditionFromMasterEditionViaVaultProxy {
    /// The `mint_new_edition_from_master_edition_via_token_args` wire field.
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
/// Bounded codec for [`MintNewEditionFromMasterEditionViaVaultProxy`].
#[derive(Clone, Copy, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyCodec;
impl Encoder<MintNewEditionFromMasterEditionViaVaultProxy>
    for MintNewEditionFromMasterEditionViaVaultProxyCodec
{
    fn encoded_len(
        &self,
        value: &MintNewEditionFromMasterEditionViaVaultProxy,
    ) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(
                MintNewEditionFromMasterEditionViaTokenArgsCodec
                    .encoded_len(&value.mint_new_edition_from_master_edition_via_token_args)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &MintNewEditionFromMasterEditionViaVaultProxy,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_13]).write(&(), writer)?;
        MintNewEditionFromMasterEditionViaTokenArgsCodec.write(
            &value.mint_new_edition_from_master_edition_via_token_args,
            writer,
        )?;
        Ok(())
    }
}
impl Decoder<MintNewEditionFromMasterEditionViaVaultProxy>
    for MintNewEditionFromMasterEditionViaVaultProxyCodec
{
    fn read(
        &self,
        reader: &mut Reader<'_>,
    ) -> Result<MintNewEditionFromMasterEditionViaVaultProxy, Error> {
        Constant(&[INSTRUCTION_TAG_13]).read(reader)?;
        Ok(MintNewEditionFromMasterEditionViaVaultProxy {
            mint_new_edition_from_master_edition_via_token_args:
                MintNewEditionFromMasterEditionViaTokenArgsCodec.read(reader)?,
        })
    }
}
impl MintNewEditionFromMasterEditionViaVaultProxy {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 16 || accounts.len() > 17 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* new_metadata */ (true, Some(false), false),
                /* new_edition */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), false),
                /* new_mint */ (true, Some(false), false),
                /* edition_mark_pda */ (true, Some(false), false),
                /* new_mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* vault_authority */ (false, Some(true), false),
                /* safety_deposit_store */ (false, Some(false), false),
                /* safety_deposit_box */ (false, Some(false), false),
                /* vault */ (false, Some(false), false),
                /* new_metadata_update_authority */ (false, Some(false), false),
                /* metadata */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* token_vault_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data =
            MintNewEditionFromMasterEditionViaVaultProxyCodec.encode(self, limits.max_bytes)?;
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
        MintNewEditionFromMasterEditionViaVaultProxyCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `PuffMetadata`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PuffMetadata {}
/// Bounded codec for [`PuffMetadata`].
#[derive(Clone, Copy, Debug)]
pub struct PuffMetadataCodec;
impl Encoder<PuffMetadata> for PuffMetadataCodec {
    fn encoded_len(&self, value: &PuffMetadata) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &PuffMetadata, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_14]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<PuffMetadata> for PuffMetadataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<PuffMetadata, Error> {
        Constant(&[INSTRUCTION_TAG_14]).read(reader)?;
        let _ = reader;
        Ok(PuffMetadata {})
    }
}
impl PuffMetadata {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 1 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([/* metadata */ (true, Some(false), false)])
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
        let data = PuffMetadataCodec.encode(self, limits.max_bytes)?;
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
        PuffMetadataCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UpdateMetadataAccountV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateMetadataAccountV2 {
    /// The `data` wire field.
    pub data: Option<DataV2>,
    /// The `new_update_authority` wire field.
    pub new_update_authority: Option<Address>,
    /// The `primary_sale_happened` wire field.
    pub primary_sale_happened: Option<bool>,
    /// The `is_mutable` wire field.
    pub is_mutable: Option<bool>,
}
/// Bounded codec for [`UpdateMetadataAccountV2`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateMetadataAccountV2Codec;
impl Encoder<UpdateMetadataAccountV2> for UpdateMetadataAccountV2Codec {
    fn encoded_len(&self, value: &UpdateMetadataAccountV2) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(OptionCodec::u8(DataV2Codec).encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ))
                .encoded_len(&value.new_update_authority)?,
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
    fn write(&self, value: &UpdateMetadataAccountV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_15]).write(&(), writer)?;
        OptionCodec::u8(DataV2Codec).write(&value.data, writer)?;
        OptionCodec::u8(Mapped::new(
            FixedBytes::<32>,
            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
        ))
        .write(&value.new_update_authority, writer)?;
        OptionCodec::u8(Bool).write(&value.primary_sale_happened, writer)?;
        OptionCodec::u8(Bool).write(&value.is_mutable, writer)?;
        Ok(())
    }
}
impl Decoder<UpdateMetadataAccountV2> for UpdateMetadataAccountV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpdateMetadataAccountV2, Error> {
        Constant(&[INSTRUCTION_TAG_15]).read(reader)?;
        Ok(UpdateMetadataAccountV2 {
            data: OptionCodec::u8(DataV2Codec).read(reader)?,
            new_update_authority: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ))
            .read(reader)?,
            primary_sale_happened: OptionCodec::u8(Bool).read(reader)?,
            is_mutable: OptionCodec::u8(Bool).read(reader)?,
        })
    }
}
impl UpdateMetadataAccountV2 {
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
                /* metadata */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
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
        let data = UpdateMetadataAccountV2Codec.encode(self, limits.max_bytes)?;
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
        UpdateMetadataAccountV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateMetadataAccountV2`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateMetadataAccountV2 {}
/// Bounded codec for [`CreateMetadataAccountV2`].
#[derive(Clone, Copy, Debug)]
pub struct CreateMetadataAccountV2Codec;
impl Encoder<CreateMetadataAccountV2> for CreateMetadataAccountV2Codec {
    fn encoded_len(&self, value: &CreateMetadataAccountV2) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CreateMetadataAccountV2, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_16]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CreateMetadataAccountV2> for CreateMetadataAccountV2Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateMetadataAccountV2, Error> {
        Constant(&[INSTRUCTION_TAG_16]).read(reader)?;
        let _ = reader;
        Ok(CreateMetadataAccountV2 {})
    }
}
impl CreateMetadataAccountV2 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = CreateMetadataAccountV2Codec.encode(self, limits.max_bytes)?;
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
        CreateMetadataAccountV2Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateMasterEditionV3`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateMasterEditionV3 {
    /// The `max_supply` wire field.
    pub max_supply: Option<u64>,
}
/// Bounded codec for [`CreateMasterEditionV3`].
#[derive(Clone, Copy, Debug)]
pub struct CreateMasterEditionV3Codec;
impl Encoder<CreateMasterEditionV3> for CreateMasterEditionV3Codec {
    fn encoded_len(&self, value: &CreateMasterEditionV3) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(OptionCodec::u8(Number::<u64>::LE).encoded_len(&value.max_supply)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateMasterEditionV3, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_17]).write(&(), writer)?;
        OptionCodec::u8(Number::<u64>::LE).write(&value.max_supply, writer)?;
        Ok(())
    }
}
impl Decoder<CreateMasterEditionV3> for CreateMasterEditionV3Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateMasterEditionV3, Error> {
        Constant(&[INSTRUCTION_TAG_17]).read(reader)?;
        Ok(CreateMasterEditionV3 {
            max_supply: OptionCodec::u8(Number::<u64>::LE).read(reader)?,
        })
    }
}
impl CreateMasterEditionV3 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 || accounts.len() > 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* edition */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
                /* mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* metadata */ (true, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = CreateMasterEditionV3Codec.encode(self, limits.max_bytes)?;
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
        CreateMasterEditionV3Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VerifyCollection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyCollection {}
/// Bounded codec for [`VerifyCollection`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyCollectionCodec;
impl Encoder<VerifyCollection> for VerifyCollectionCodec {
    fn encoded_len(&self, value: &VerifyCollection) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &VerifyCollection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_18]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<VerifyCollection> for VerifyCollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifyCollection, Error> {
        Constant(&[INSTRUCTION_TAG_18]).read(reader)?;
        let _ = reader;
        Ok(VerifyCollection {})
    }
}
impl VerifyCollection {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* collection_authority */ (true, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection */ (false, Some(false), false),
                /* collection_master_edition_account */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
/// Validated data for `Utilize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Utilize {
    /// The `number_of_uses` wire field.
    pub number_of_uses: u64,
}
/// Bounded codec for [`Utilize`].
#[derive(Clone, Copy, Debug)]
pub struct UtilizeCodec;
impl Encoder<Utilize> for UtilizeCodec {
    fn encoded_len(&self, value: &Utilize) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.number_of_uses)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Utilize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_19]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.number_of_uses, writer)?;
        Ok(())
    }
}
impl Decoder<Utilize> for UtilizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Utilize, Error> {
        Constant(&[INSTRUCTION_TAG_19]).read(reader)?;
        Ok(Utilize {
            number_of_uses: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl Utilize {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 9 || accounts.len() > 11 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* token_account */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* use_authority */ (true, Some(true), false),
                /* owner */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* ata_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
                /* use_authority_record */ (true, Some(false), false),
                /* burner */ (false, Some(false), false),
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
        let data = UtilizeCodec.encode(self, limits.max_bytes)?;
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
        UtilizeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ApproveUseAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproveUseAuthority {
    /// The `number_of_uses` wire field.
    pub number_of_uses: u64,
}
/// Bounded codec for [`ApproveUseAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveUseAuthorityCodec;
impl Encoder<ApproveUseAuthority> for ApproveUseAuthorityCodec {
    fn encoded_len(&self, value: &ApproveUseAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.number_of_uses)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ApproveUseAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_20]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.number_of_uses, writer)?;
        Ok(())
    }
}
impl Decoder<ApproveUseAuthority> for ApproveUseAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApproveUseAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_20]).read(reader)?;
        Ok(ApproveUseAuthority {
            number_of_uses: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl ApproveUseAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 10 || accounts.len() > 11 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* use_authority_record */ (true, Some(false), false),
                /* owner */ (true, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* user */ (false, Some(false), false),
                /* owner_token_account */ (true, Some(false), false),
                /* metadata */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* burner */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = ApproveUseAuthorityCodec.encode(self, limits.max_bytes)?;
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
        ApproveUseAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeUseAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeUseAuthority {}
/// Bounded codec for [`RevokeUseAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeUseAuthorityCodec;
impl Encoder<RevokeUseAuthority> for RevokeUseAuthorityCodec {
    fn encoded_len(&self, value: &RevokeUseAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &RevokeUseAuthority, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_21]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RevokeUseAuthority> for RevokeUseAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeUseAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_21]).read(reader)?;
        let _ = reader;
        Ok(RevokeUseAuthority {})
    }
}
impl RevokeUseAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 || accounts.len() > 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* use_authority_record */ (true, Some(false), false),
                /* owner */ (true, Some(true), false),
                /* user */ (false, Some(false), false),
                /* owner_token_account */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* metadata */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = RevokeUseAuthorityCodec.encode(self, limits.max_bytes)?;
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
        RevokeUseAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UnverifyCollection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnverifyCollection {}
/// Bounded codec for [`UnverifyCollection`].
#[derive(Clone, Copy, Debug)]
pub struct UnverifyCollectionCodec;
impl Encoder<UnverifyCollection> for UnverifyCollectionCodec {
    fn encoded_len(&self, value: &UnverifyCollection) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &UnverifyCollection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_22]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<UnverifyCollection> for UnverifyCollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnverifyCollection, Error> {
        Constant(&[INSTRUCTION_TAG_22]).read(reader)?;
        let _ = reader;
        Ok(UnverifyCollection {})
    }
}
impl UnverifyCollection {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 5 || accounts.len() > 6 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* collection_authority */ (true, Some(true), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection */ (false, Some(false), false),
                /* collection_master_edition_account */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
/// Validated data for `ApproveCollectionAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproveCollectionAuthority {}
/// Bounded codec for [`ApproveCollectionAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct ApproveCollectionAuthorityCodec;
impl Encoder<ApproveCollectionAuthority> for ApproveCollectionAuthorityCodec {
    fn encoded_len(&self, value: &ApproveCollectionAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &ApproveCollectionAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_23]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ApproveCollectionAuthority> for ApproveCollectionAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ApproveCollectionAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_23]).read(reader)?;
        let _ = reader;
        Ok(ApproveCollectionAuthority {})
    }
}
impl ApproveCollectionAuthority {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 7 || accounts.len() > 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* collection_authority_record */ (true, Some(false), false),
                /* new_collection_authority */ (false, Some(false), false),
                /* update_authority */ (true, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* metadata */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = ApproveCollectionAuthorityCodec.encode(self, limits.max_bytes)?;
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
        ApproveCollectionAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RevokeCollectionAuthority`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokeCollectionAuthority {}
/// Bounded codec for [`RevokeCollectionAuthority`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeCollectionAuthorityCodec;
impl Encoder<RevokeCollectionAuthority> for RevokeCollectionAuthorityCodec {
    fn encoded_len(&self, value: &RevokeCollectionAuthority) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &RevokeCollectionAuthority,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_24]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RevokeCollectionAuthority> for RevokeCollectionAuthorityCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RevokeCollectionAuthority, Error> {
        Constant(&[INSTRUCTION_TAG_24]).read(reader)?;
        let _ = reader;
        Ok(RevokeCollectionAuthority {})
    }
}
impl RevokeCollectionAuthority {
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
                /* collection_authority_record */ (true, Some(false), false),
                /* delegate_authority */ (true, Some(false), false),
                /* revoke_authority */ (true, Some(true), false),
                /* metadata */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
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
        let data = RevokeCollectionAuthorityCodec.encode(self, limits.max_bytes)?;
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
        RevokeCollectionAuthorityCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetAndVerifyCollection`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAndVerifyCollection {}
/// Bounded codec for [`SetAndVerifyCollection`].
#[derive(Clone, Copy, Debug)]
pub struct SetAndVerifyCollectionCodec;
impl Encoder<SetAndVerifyCollection> for SetAndVerifyCollectionCodec {
    fn encoded_len(&self, value: &SetAndVerifyCollection) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SetAndVerifyCollection, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_25]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SetAndVerifyCollection> for SetAndVerifyCollectionCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAndVerifyCollection, Error> {
        Constant(&[INSTRUCTION_TAG_25]).read(reader)?;
        let _ = reader;
        Ok(SetAndVerifyCollection {})
    }
}
impl SetAndVerifyCollection {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 7 || accounts.len() > 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* collection_authority */ (true, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection */ (false, Some(false), false),
                /* collection_master_edition_account */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
/// Validated data for `FreezeDelegatedAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreezeDelegatedAccount {}
/// Bounded codec for [`FreezeDelegatedAccount`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeDelegatedAccountCodec;
impl Encoder<FreezeDelegatedAccount> for FreezeDelegatedAccountCodec {
    fn encoded_len(&self, value: &FreezeDelegatedAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &FreezeDelegatedAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_26]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<FreezeDelegatedAccount> for FreezeDelegatedAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeDelegatedAccount, Error> {
        Constant(&[INSTRUCTION_TAG_26]).read(reader)?;
        let _ = reader;
        Ok(FreezeDelegatedAccount {})
    }
}
impl FreezeDelegatedAccount {
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
                /* delegate */ (true, Some(true), false),
                /* token_account */ (true, Some(false), false),
                /* edition */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
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
        let data = FreezeDelegatedAccountCodec.encode(self, limits.max_bytes)?;
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
        FreezeDelegatedAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `ThawDelegatedAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThawDelegatedAccount {}
/// Bounded codec for [`ThawDelegatedAccount`].
#[derive(Clone, Copy, Debug)]
pub struct ThawDelegatedAccountCodec;
impl Encoder<ThawDelegatedAccount> for ThawDelegatedAccountCodec {
    fn encoded_len(&self, value: &ThawDelegatedAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &ThawDelegatedAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_27]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<ThawDelegatedAccount> for ThawDelegatedAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ThawDelegatedAccount, Error> {
        Constant(&[INSTRUCTION_TAG_27]).read(reader)?;
        let _ = reader;
        Ok(ThawDelegatedAccount {})
    }
}
impl ThawDelegatedAccount {
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
                /* delegate */ (true, Some(true), false),
                /* token_account */ (true, Some(false), false),
                /* edition */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
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
        let data = ThawDelegatedAccountCodec.encode(self, limits.max_bytes)?;
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
        ThawDelegatedAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `RemoveCreatorVerification`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveCreatorVerification {}
/// Bounded codec for [`RemoveCreatorVerification`].
#[derive(Clone, Copy, Debug)]
pub struct RemoveCreatorVerificationCodec;
impl Encoder<RemoveCreatorVerification> for RemoveCreatorVerificationCodec {
    fn encoded_len(&self, value: &RemoveCreatorVerification) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &RemoveCreatorVerification,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_28]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<RemoveCreatorVerification> for RemoveCreatorVerificationCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<RemoveCreatorVerification, Error> {
        Constant(&[INSTRUCTION_TAG_28]).read(reader)?;
        let _ = reader;
        Ok(RemoveCreatorVerification {})
    }
}
impl RemoveCreatorVerification {
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
                /* metadata */ (true, Some(false), false),
                /* creator */ (false, Some(true), false),
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
        let data = RemoveCreatorVerificationCodec.encode(self, limits.max_bytes)?;
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
        RemoveCreatorVerificationCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BurnNft`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnNft {}
/// Bounded codec for [`BurnNft`].
#[derive(Clone, Copy, Debug)]
pub struct BurnNftCodec;
impl Encoder<BurnNft> for BurnNftCodec {
    fn encoded_len(&self, value: &BurnNft) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &BurnNft, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_29]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<BurnNft> for BurnNftCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnNft, Error> {
        Constant(&[INSTRUCTION_TAG_29]).read(reader)?;
        let _ = reader;
        Ok(BurnNft {})
    }
}
impl BurnNft {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* owner */ (true, Some(true), false),
                /* mint */ (true, Some(false), false),
                /* token_account */ (true, Some(false), false),
                /* master_edition_account */ (true, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
                /* collection_metadata */ (true, Some(false), false),
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
        let data = BurnNftCodec.encode(self, limits.max_bytes)?;
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
        BurnNftCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `VerifySizedCollectionItem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifySizedCollectionItem {}
/// Bounded codec for [`VerifySizedCollectionItem`].
#[derive(Clone, Copy, Debug)]
pub struct VerifySizedCollectionItemCodec;
impl Encoder<VerifySizedCollectionItem> for VerifySizedCollectionItemCodec {
    fn encoded_len(&self, value: &VerifySizedCollectionItem) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &VerifySizedCollectionItem,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_30]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<VerifySizedCollectionItem> for VerifySizedCollectionItemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<VerifySizedCollectionItem, Error> {
        Constant(&[INSTRUCTION_TAG_30]).read(reader)?;
        let _ = reader;
        Ok(VerifySizedCollectionItem {})
    }
}
impl VerifySizedCollectionItem {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection */ (true, Some(false), false),
                /* collection_master_edition_account */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
        let data = VerifySizedCollectionItemCodec.encode(self, limits.max_bytes)?;
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
        VerifySizedCollectionItemCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `UnverifySizedCollectionItem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnverifySizedCollectionItem {}
/// Bounded codec for [`UnverifySizedCollectionItem`].
#[derive(Clone, Copy, Debug)]
pub struct UnverifySizedCollectionItemCodec;
impl Encoder<UnverifySizedCollectionItem> for UnverifySizedCollectionItemCodec {
    fn encoded_len(&self, value: &UnverifySizedCollectionItem) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &UnverifySizedCollectionItem,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_31]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<UnverifySizedCollectionItem> for UnverifySizedCollectionItemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UnverifySizedCollectionItem, Error> {
        Constant(&[INSTRUCTION_TAG_31]).read(reader)?;
        let _ = reader;
        Ok(UnverifySizedCollectionItem {})
    }
}
impl UnverifySizedCollectionItem {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection */ (true, Some(false), false),
                /* collection_master_edition_account */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
        let data = UnverifySizedCollectionItemCodec.encode(self, limits.max_bytes)?;
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
        UnverifySizedCollectionItemCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetAndVerifySizedCollectionItem`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetAndVerifySizedCollectionItem {}
/// Bounded codec for [`SetAndVerifySizedCollectionItem`].
#[derive(Clone, Copy, Debug)]
pub struct SetAndVerifySizedCollectionItemCodec;
impl Encoder<SetAndVerifySizedCollectionItem> for SetAndVerifySizedCollectionItemCodec {
    fn encoded_len(&self, value: &SetAndVerifySizedCollectionItem) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(
        &self,
        value: &SetAndVerifySizedCollectionItem,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_32]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SetAndVerifySizedCollectionItem> for SetAndVerifySizedCollectionItemCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetAndVerifySizedCollectionItem, Error> {
        Constant(&[INSTRUCTION_TAG_32]).read(reader)?;
        let _ = reader;
        Ok(SetAndVerifySizedCollectionItem {})
    }
}
impl SetAndVerifySizedCollectionItem {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 7 || accounts.len() > 8 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection */ (true, Some(false), false),
                /* collection_master_edition_account */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
        let data = SetAndVerifySizedCollectionItemCodec.encode(self, limits.max_bytes)?;
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
        SetAndVerifySizedCollectionItemCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateMetadataAccountV3`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateMetadataAccountV3 {
    /// The `data` wire field.
    pub data: DataV2,
    /// The `is_mutable` wire field.
    pub is_mutable: bool,
    /// The `collection_details` wire field.
    pub collection_details: Option<CollectionDetails>,
}
/// Bounded codec for [`CreateMetadataAccountV3`].
#[derive(Clone, Copy, Debug)]
pub struct CreateMetadataAccountV3Codec;
impl Encoder<CreateMetadataAccountV3> for CreateMetadataAccountV3Codec {
    fn encoded_len(&self, value: &CreateMetadataAccountV3) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(DataV2Codec.encoded_len(&value.data)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bool.encoded_len(&value.is_mutable)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OptionCodec::u8(CollectionDetailsCodec).encoded_len(&value.collection_details)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateMetadataAccountV3, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_33]).write(&(), writer)?;
        DataV2Codec.write(&value.data, writer)?;
        Bool.write(&value.is_mutable, writer)?;
        OptionCodec::u8(CollectionDetailsCodec).write(&value.collection_details, writer)?;
        Ok(())
    }
}
impl Decoder<CreateMetadataAccountV3> for CreateMetadataAccountV3Codec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateMetadataAccountV3, Error> {
        Constant(&[INSTRUCTION_TAG_33]).read(reader)?;
        Ok(CreateMetadataAccountV3 {
            data: DataV2Codec.read(reader)?,
            is_mutable: Bool.read(reader)?,
            collection_details: OptionCodec::u8(CollectionDetailsCodec).read(reader)?,
        })
    }
}
impl CreateMetadataAccountV3 {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 6 || accounts.len() > 7 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* mint_authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* update_authority */ (false, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* rent */ (false, Some(false), false),
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
        let data = CreateMetadataAccountV3Codec.encode(self, limits.max_bytes)?;
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
        CreateMetadataAccountV3Codec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetCollectionSize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetCollectionSize {
    /// The `set_collection_size_args` wire field.
    pub set_collection_size_args: SetCollectionSizeArgs,
}
/// Bounded codec for [`SetCollectionSize`].
#[derive(Clone, Copy, Debug)]
pub struct SetCollectionSizeCodec;
impl Encoder<SetCollectionSize> for SetCollectionSizeCodec {
    fn encoded_len(&self, value: &SetCollectionSize) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(SetCollectionSizeArgsCodec.encoded_len(&value.set_collection_size_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &SetCollectionSize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_34]).write(&(), writer)?;
        SetCollectionSizeArgsCodec.write(&value.set_collection_size_args, writer)?;
        Ok(())
    }
}
impl Decoder<SetCollectionSize> for SetCollectionSizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetCollectionSize, Error> {
        Constant(&[INSTRUCTION_TAG_34]).read(reader)?;
        Ok(SetCollectionSize {
            set_collection_size_args: SetCollectionSizeArgsCodec.read(reader)?,
        })
    }
}
impl SetCollectionSize {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 || accounts.len() > 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* collection_metadata */ (true, Some(false), false),
                /* collection_authority */ (true, Some(true), false),
                /* collection_mint */ (false, Some(false), false),
                /* collection_authority_record */ (false, Some(false), false),
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
        let data = SetCollectionSizeCodec.encode(self, limits.max_bytes)?;
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
        SetCollectionSizeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `SetTokenStandard`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetTokenStandard {}
/// Bounded codec for [`SetTokenStandard`].
#[derive(Clone, Copy, Debug)]
pub struct SetTokenStandardCodec;
impl Encoder<SetTokenStandard> for SetTokenStandardCodec {
    fn encoded_len(&self, value: &SetTokenStandard) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &SetTokenStandard, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_35]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<SetTokenStandard> for SetTokenStandardCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<SetTokenStandard, Error> {
        Constant(&[INSTRUCTION_TAG_35]).read(reader)?;
        let _ = reader;
        Ok(SetTokenStandard {})
    }
}
impl SetTokenStandard {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 3 || accounts.len() > 4 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* update_authority */ (false, Some(true), false),
                /* mint */ (false, Some(false), false),
                /* edition */ (false, Some(false), false),
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
        let data = SetTokenStandardCodec.encode(self, limits.max_bytes)?;
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
        SetTokenStandardCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BubblegumSetCollectionSize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BubblegumSetCollectionSize {
    /// The `set_collection_size_args` wire field.
    pub set_collection_size_args: SetCollectionSizeArgs,
}
/// Bounded codec for [`BubblegumSetCollectionSize`].
#[derive(Clone, Copy, Debug)]
pub struct BubblegumSetCollectionSizeCodec;
impl Encoder<BubblegumSetCollectionSize> for BubblegumSetCollectionSizeCodec {
    fn encoded_len(&self, value: &BubblegumSetCollectionSize) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(SetCollectionSizeArgsCodec.encoded_len(&value.set_collection_size_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(
        &self,
        value: &BubblegumSetCollectionSize,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_36]).write(&(), writer)?;
        SetCollectionSizeArgsCodec.write(&value.set_collection_size_args, writer)?;
        Ok(())
    }
}
impl Decoder<BubblegumSetCollectionSize> for BubblegumSetCollectionSizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BubblegumSetCollectionSize, Error> {
        Constant(&[INSTRUCTION_TAG_36]).read(reader)?;
        Ok(BubblegumSetCollectionSize {
            set_collection_size_args: SetCollectionSizeArgsCodec.read(reader)?,
        })
    }
}
impl BubblegumSetCollectionSize {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 4 || accounts.len() > 5 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* collection_metadata */ (true, Some(false), false),
                /* collection_authority */ (false, Some(true), false),
                /* collection_mint */ (false, Some(false), false),
                /* bubblegum_signer */ (false, Some(true), false),
                /* collection_authority_record */ (false, Some(false), false),
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
        let data = BubblegumSetCollectionSizeCodec.encode(self, limits.max_bytes)?;
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
        BubblegumSetCollectionSizeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `BurnEditionNft`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnEditionNft {}
/// Bounded codec for [`BurnEditionNft`].
#[derive(Clone, Copy, Debug)]
pub struct BurnEditionNftCodec;
impl Encoder<BurnEditionNft> for BurnEditionNftCodec {
    fn encoded_len(&self, value: &BurnEditionNft) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &BurnEditionNft, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_37]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<BurnEditionNft> for BurnEditionNftCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<BurnEditionNft, Error> {
        Constant(&[INSTRUCTION_TAG_37]).read(reader)?;
        let _ = reader;
        Ok(BurnEditionNft {})
    }
}
impl BurnEditionNft {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 10 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* owner */ (true, Some(true), false),
                /* print_edition_mint */ (true, Some(false), false),
                /* master_edition_mint */ (false, Some(false), false),
                /* print_edition_token_account */ (true, Some(false), false),
                /* master_edition_token_account */ (false, Some(false), false),
                /* master_edition_account */ (true, Some(false), false),
                /* print_edition_account */ (true, Some(false), false),
                /* edition_marker_account */ (true, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
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
        let data = BurnEditionNftCodec.encode(self, limits.max_bytes)?;
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
        BurnEditionNftCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CreateEscrowAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateEscrowAccount {}
/// Bounded codec for [`CreateEscrowAccount`].
#[derive(Clone, Copy, Debug)]
pub struct CreateEscrowAccountCodec;
impl Encoder<CreateEscrowAccount> for CreateEscrowAccountCodec {
    fn encoded_len(&self, value: &CreateEscrowAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CreateEscrowAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_38]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CreateEscrowAccount> for CreateEscrowAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateEscrowAccount, Error> {
        Constant(&[INSTRUCTION_TAG_38]).read(reader)?;
        let _ = reader;
        Ok(CreateEscrowAccount {})
    }
}
impl CreateEscrowAccount {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 8 || accounts.len() > 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* escrow */ (true, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* token_account */ (false, Some(false), false),
                /* edition */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* authority */ (false, Some(true), false),
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
        let data = CreateEscrowAccountCodec.encode(self, limits.max_bytes)?;
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
        CreateEscrowAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseEscrowAccount`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseEscrowAccount {}
/// Bounded codec for [`CloseEscrowAccount`].
#[derive(Clone, Copy, Debug)]
pub struct CloseEscrowAccountCodec;
impl Encoder<CloseEscrowAccount> for CloseEscrowAccountCodec {
    fn encoded_len(&self, value: &CloseEscrowAccount) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseEscrowAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_39]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseEscrowAccount> for CloseEscrowAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseEscrowAccount, Error> {
        Constant(&[INSTRUCTION_TAG_39]).read(reader)?;
        let _ = reader;
        Ok(CloseEscrowAccount {})
    }
}
impl CloseEscrowAccount {
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
                /* escrow */ (true, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* token_account */ (false, Some(false), false),
                /* edition */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
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
        let data = CloseEscrowAccountCodec.encode(self, limits.max_bytes)?;
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
        CloseEscrowAccountCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `TransferOutOfEscrow`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferOutOfEscrow {
    /// The `amount` wire field.
    pub amount: u64,
}
/// Bounded codec for [`TransferOutOfEscrow`].
#[derive(Clone, Copy, Debug)]
pub struct TransferOutOfEscrowCodec;
impl Encoder<TransferOutOfEscrow> for TransferOutOfEscrowCodec {
    fn encoded_len(&self, value: &TransferOutOfEscrow) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.amount)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &TransferOutOfEscrow, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_40]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.amount, writer)?;
        Ok(())
    }
}
impl Decoder<TransferOutOfEscrow> for TransferOutOfEscrowCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TransferOutOfEscrow, Error> {
        Constant(&[INSTRUCTION_TAG_40]).read(reader)?;
        Ok(TransferOutOfEscrow {
            amount: Number::<u64>::LE.read(reader)?,
        })
    }
}
impl TransferOutOfEscrow {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() < 12 || accounts.len() > 13 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* escrow */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* attribute_mint */ (false, Some(false), false),
                /* attribute_src */ (true, Some(false), false),
                /* attribute_dst */ (true, Some(false), false),
                /* escrow_mint */ (false, Some(false), false),
                /* escrow_account */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* ata_program */ (false, Some(false), false),
                /* token_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* authority */ (false, Some(true), false),
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
        let data = TransferOutOfEscrowCodec.encode(self, limits.max_bytes)?;
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
        TransferOutOfEscrowCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Burn`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Burn {
    /// The `burn_args` wire field.
    pub burn_args: BurnArgs,
}
/// Bounded codec for [`Burn`].
#[derive(Clone, Copy, Debug)]
pub struct BurnCodec;
impl Encoder<Burn> for BurnCodec {
    fn encoded_len(&self, value: &Burn) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(BurnArgsCodec.encoded_len(&value.burn_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Burn, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_41]).write(&(), writer)?;
        BurnArgsCodec.write(&value.burn_args, writer)?;
        Ok(())
    }
}
impl Decoder<Burn> for BurnCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Burn, Error> {
        Constant(&[INSTRUCTION_TAG_41]).read(reader)?;
        Ok(Burn {
            burn_args: BurnArgsCodec.read(reader)?,
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

        if accounts.len() != 14 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* authority */ (true, Some(true), false),
                /* collection_metadata */ (true, Some(false), true),
                /* metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), true),
                /* mint */ (true, Some(false), false),
                /* token */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), true),
                /* master_edition_mint */ (false, Some(false), true),
                /* master_edition_token */ (false, Some(false), true),
                /* edition_marker */ (true, Some(false), true),
                /* token_record */ (true, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
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
/// Validated data for `Create`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Create {
    /// The `create_args` wire field.
    pub create_args: CreateArgs,
}
/// Bounded codec for [`Create`].
#[derive(Clone, Copy, Debug)]
pub struct CreateCodec;
impl Encoder<Create> for CreateCodec {
    fn encoded_len(&self, value: &Create) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(CreateArgsCodec.encoded_len(&value.create_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Create, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_42]).write(&(), writer)?;
        CreateArgsCodec.write(&value.create_args, writer)?;
        Ok(())
    }
}
impl Decoder<Create> for CreateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Create, Error> {
        Constant(&[INSTRUCTION_TAG_42]).read(reader)?;
        Ok(Create {
            create_args: CreateArgsCodec.read(reader)?,
        })
    }
}
impl Create {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 9 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* master_edition */ (true, Some(false), true),
                /* mint */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* update_authority */ (false, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), true),
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
        let data = CreateCodec.encode(self, limits.max_bytes)?;
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
        CreateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Mint`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mint {
    /// The `mint_args` wire field.
    pub mint_args: MintArgs,
}
/// Bounded codec for [`Mint`].
#[derive(Clone, Copy, Debug)]
pub struct MintCodec;
impl Encoder<Mint> for MintCodec {
    fn encoded_len(&self, value: &Mint) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(MintArgsCodec.encoded_len(&value.mint_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Mint, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_43]).write(&(), writer)?;
        MintArgsCodec.write(&value.mint_args, writer)?;
        Ok(())
    }
}
impl Decoder<Mint> for MintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Mint, Error> {
        Constant(&[INSTRUCTION_TAG_43]).read(reader)?;
        Ok(Mint {
            mint_args: MintArgsCodec.read(reader)?,
        })
    }
}
impl Mint {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 15 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* token */ (true, Some(false), false),
                /* token_owner */ (false, Some(false), true),
                /* metadata */ (false, Some(false), false),
                /* master_edition */ (true, Some(false), true),
                /* token_record */ (true, Some(false), true),
                /* mint */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
                /* delegate_record */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
                /* spl_ata_program */ (false, Some(false), false),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = MintCodec.encode(self, limits.max_bytes)?;
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
        MintCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Delegate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delegate {
    /// The `delegate_args` wire field.
    pub delegate_args: DelegateArgs,
}
/// Bounded codec for [`Delegate`].
#[derive(Clone, Copy, Debug)]
pub struct DelegateCodec;
impl Encoder<Delegate> for DelegateCodec {
    fn encoded_len(&self, value: &Delegate) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(DelegateArgsCodec.encoded_len(&value.delegate_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Delegate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_44]).write(&(), writer)?;
        DelegateArgsCodec.write(&value.delegate_args, writer)?;
        Ok(())
    }
}
impl Decoder<Delegate> for DelegateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Delegate, Error> {
        Constant(&[INSTRUCTION_TAG_44]).read(reader)?;
        Ok(Delegate {
            delegate_args: DelegateArgsCodec.read(reader)?,
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

        if accounts.len() != 14 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* delegate_record */ (true, Some(false), true),
                /* delegate */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* master_edition */ (false, Some(false), true),
                /* token_record */ (true, Some(false), true),
                /* mint */ (false, Some(false), false),
                /* token */ (true, Some(false), true),
                /* authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), true),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
/// Validated data for `Revoke`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revoke {
    /// The `revoke_args` wire field.
    pub revoke_args: RevokeArgs,
}
/// Bounded codec for [`Revoke`].
#[derive(Clone, Copy, Debug)]
pub struct RevokeCodec;
impl Encoder<Revoke> for RevokeCodec {
    fn encoded_len(&self, value: &Revoke) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(RevokeArgsCodec.encoded_len(&value.revoke_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Revoke, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_45]).write(&(), writer)?;
        RevokeArgsCodec.write(&value.revoke_args, writer)?;
        Ok(())
    }
}
impl Decoder<Revoke> for RevokeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Revoke, Error> {
        Constant(&[INSTRUCTION_TAG_45]).read(reader)?;
        Ok(Revoke {
            revoke_args: RevokeArgsCodec.read(reader)?,
        })
    }
}
impl Revoke {
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
                /* delegate_record */ (true, Some(false), true),
                /* delegate */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* master_edition */ (false, Some(false), true),
                /* token_record */ (true, Some(false), true),
                /* mint */ (false, Some(false), false),
                /* token */ (true, Some(false), true),
                /* authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), true),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = RevokeCodec.encode(self, limits.max_bytes)?;
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
        RevokeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Lock`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lock {
    /// The `lock_args` wire field.
    pub lock_args: LockArgs,
}
/// Bounded codec for [`Lock`].
#[derive(Clone, Copy, Debug)]
pub struct LockCodec;
impl Encoder<Lock> for LockCodec {
    fn encoded_len(&self, value: &Lock) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(LockArgsCodec.encoded_len(&value.lock_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Lock, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_46]).write(&(), writer)?;
        LockArgsCodec.write(&value.lock_args, writer)?;
        Ok(())
    }
}
impl Decoder<Lock> for LockCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Lock, Error> {
        Constant(&[INSTRUCTION_TAG_46]).read(reader)?;
        Ok(Lock {
            lock_args: LockArgsCodec.read(reader)?,
        })
    }
}
impl Lock {
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
                /* authority */ (false, Some(true), false),
                /* token_owner */ (false, Some(false), true),
                /* token */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* edition */ (false, Some(false), true),
                /* token_record */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), true),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = LockCodec.encode(self, limits.max_bytes)?;
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
        LockCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Unlock`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unlock {
    /// The `unlock_args` wire field.
    pub unlock_args: UnlockArgs,
}
/// Bounded codec for [`Unlock`].
#[derive(Clone, Copy, Debug)]
pub struct UnlockCodec;
impl Encoder<Unlock> for UnlockCodec {
    fn encoded_len(&self, value: &Unlock) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(UnlockArgsCodec.encoded_len(&value.unlock_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Unlock, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_47]).write(&(), writer)?;
        UnlockArgsCodec.write(&value.unlock_args, writer)?;
        Ok(())
    }
}
impl Decoder<Unlock> for UnlockCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Unlock, Error> {
        Constant(&[INSTRUCTION_TAG_47]).read(reader)?;
        Ok(Unlock {
            unlock_args: UnlockArgsCodec.read(reader)?,
        })
    }
}
impl Unlock {
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
                /* authority */ (false, Some(true), false),
                /* token_owner */ (false, Some(false), true),
                /* token */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* edition */ (false, Some(false), true),
                /* token_record */ (true, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), true),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = UnlockCodec.encode(self, limits.max_bytes)?;
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
        UnlockCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Migrate`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Migrate {}
/// Bounded codec for [`Migrate`].
#[derive(Clone, Copy, Debug)]
pub struct MigrateCodec;
impl Encoder<Migrate> for MigrateCodec {
    fn encoded_len(&self, value: &Migrate) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Migrate, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_48]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Migrate> for MigrateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Migrate, Error> {
        Constant(&[INSTRUCTION_TAG_48]).read(reader)?;
        let _ = reader;
        Ok(Migrate {})
    }
}
impl Migrate {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 15 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), false),
                /* token */ (true, Some(false), false),
                /* token_owner */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), false),
                /* collection_metadata */ (false, Some(false), false),
                /* delegate_record */ (false, Some(false), false),
                /* token_record */ (true, Some(false), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = MigrateCodec.encode(self, limits.max_bytes)?;
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
        MigrateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Transfer`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    /// The `transfer_args` wire field.
    pub transfer_args: TransferArgs,
}
/// Bounded codec for [`Transfer`].
#[derive(Clone, Copy, Debug)]
pub struct TransferCodec;
impl Encoder<Transfer> for TransferCodec {
    fn encoded_len(&self, value: &Transfer) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(TransferArgsCodec.encoded_len(&value.transfer_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Transfer, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_49]).write(&(), writer)?;
        TransferArgsCodec.write(&value.transfer_args, writer)?;
        Ok(())
    }
}
impl Decoder<Transfer> for TransferCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Transfer, Error> {
        Constant(&[INSTRUCTION_TAG_49]).read(reader)?;
        Ok(Transfer {
            transfer_args: TransferArgsCodec.read(reader)?,
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

        if accounts.len() != 17 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* token */ (true, Some(false), false),
                /* token_owner */ (false, Some(false), false),
                /* destination */ (true, Some(false), false),
                /* destination_owner */ (false, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* edition */ (false, Some(false), true),
                /* owner_token_record */ (true, Some(false), true),
                /* destination_token_record */ (true, Some(false), true),
                /* authority */ (false, Some(true), false),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
                /* spl_ata_program */ (false, Some(false), false),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
/// Validated data for `Update`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Update {
    /// The `update_args` wire field.
    pub update_args: UpdateArgs,
}
/// Bounded codec for [`Update`].
#[derive(Clone, Copy, Debug)]
pub struct UpdateCodec;
impl Encoder<Update> for UpdateCodec {
    fn encoded_len(&self, value: &Update) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(UpdateArgsCodec.encoded_len(&value.update_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Update, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_50]).write(&(), writer)?;
        UpdateArgsCodec.write(&value.update_args, writer)?;
        Ok(())
    }
}
impl Decoder<Update> for UpdateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Update, Error> {
        Constant(&[INSTRUCTION_TAG_50]).read(reader)?;
        Ok(Update {
            update_args: UpdateArgsCodec.read(reader)?,
        })
    }
}
impl Update {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 11 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* authority */ (false, Some(true), false),
                /* delegate_record */ (false, Some(false), true),
                /* token */ (false, Some(false), true),
                /* mint */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* edition */ (false, Some(false), true),
                /* payer */ (true, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = UpdateCodec.encode(self, limits.max_bytes)?;
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
        UpdateCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Use`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Use {
    /// The `use_args` wire field.
    pub use_args: UseArgs,
}
/// Bounded codec for [`Use`].
#[derive(Clone, Copy, Debug)]
pub struct UseCodec;
impl Encoder<Use> for UseCodec {
    fn encoded_len(&self, value: &Use) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(UseArgsCodec.encoded_len(&value.use_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Use, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_51]).write(&(), writer)?;
        UseArgsCodec.write(&value.use_args, writer)?;
        Ok(())
    }
}
impl Decoder<Use> for UseCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Use, Error> {
        Constant(&[INSTRUCTION_TAG_51]).read(reader)?;
        Ok(Use {
            use_args: UseArgsCodec.read(reader)?,
        })
    }
}
impl Use {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 12 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* authority */ (false, Some(true), false),
                /* delegate_record */ (true, Some(false), true),
                /* token */ (true, Some(false), true),
                /* mint */ (false, Some(false), false),
                /* metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), true),
                /* payer */ (false, Some(true), false),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), true),
                /* authorization_rules_program */ (false, Some(false), true),
                /* authorization_rules */ (false, Some(false), true),
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
        let data = UseCodec.encode(self, limits.max_bytes)?;
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
        UseCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Verify`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verify {
    /// The `verification_args` wire field.
    pub verification_args: VerificationArgs,
}
/// Bounded codec for [`Verify`].
#[derive(Clone, Copy, Debug)]
pub struct VerifyCodec;
impl Encoder<Verify> for VerifyCodec {
    fn encoded_len(&self, value: &Verify) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(VerificationArgsCodec.encoded_len(&value.verification_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Verify, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_52]).write(&(), writer)?;
        VerificationArgsCodec.write(&value.verification_args, writer)?;
        Ok(())
    }
}
impl Decoder<Verify> for VerifyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Verify, Error> {
        Constant(&[INSTRUCTION_TAG_52]).read(reader)?;
        Ok(Verify {
            verification_args: VerificationArgsCodec.read(reader)?,
        })
    }
}
impl Verify {
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
                /* authority */ (false, Some(true), false),
                /* delegate_record */ (false, Some(false), true),
                /* metadata */ (true, Some(false), false),
                /* collection_mint */ (false, Some(false), true),
                /* collection_metadata */ (true, Some(false), true),
                /* collection_master_edition */ (false, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
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
        let data = VerifyCodec.encode(self, limits.max_bytes)?;
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
        VerifyCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Unverify`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unverify {
    /// The `verification_args` wire field.
    pub verification_args: VerificationArgs,
}
/// Bounded codec for [`Unverify`].
#[derive(Clone, Copy, Debug)]
pub struct UnverifyCodec;
impl Encoder<Unverify> for UnverifyCodec {
    fn encoded_len(&self, value: &Unverify) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(VerificationArgsCodec.encoded_len(&value.verification_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Unverify, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_53]).write(&(), writer)?;
        VerificationArgsCodec.write(&value.verification_args, writer)?;
        Ok(())
    }
}
impl Decoder<Unverify> for UnverifyCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Unverify, Error> {
        Constant(&[INSTRUCTION_TAG_53]).read(reader)?;
        Ok(Unverify {
            verification_args: VerificationArgsCodec.read(reader)?,
        })
    }
}
impl Unverify {
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
                /* authority */ (false, Some(true), false),
                /* delegate_record */ (false, Some(false), true),
                /* metadata */ (true, Some(false), false),
                /* collection_mint */ (false, Some(false), true),
                /* collection_metadata */ (true, Some(false), true),
                /* system_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
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
        let data = UnverifyCodec.encode(self, limits.max_bytes)?;
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
        UnverifyCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Collect`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Collect {}
/// Bounded codec for [`Collect`].
#[derive(Clone, Copy, Debug)]
pub struct CollectCodec;
impl Encoder<Collect> for CollectCodec {
    fn encoded_len(&self, value: &Collect) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Collect, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_54]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Collect> for CollectCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Collect, Error> {
        Constant(&[INSTRUCTION_TAG_54]).read(reader)?;
        let _ = reader;
        Ok(Collect {})
    }
}
impl Collect {
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
                /* authority */ (false, Some(true), false),
                /* recipient */ (false, Some(false), false),
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
        let data = CollectCodec.encode(self, limits.max_bytes)?;
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
        CollectCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Print`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Print {
    /// The `print_args` wire field.
    pub print_args: PrintArgs,
}
/// Bounded codec for [`Print`].
#[derive(Clone, Copy, Debug)]
pub struct PrintCodec;
impl Encoder<Print> for PrintCodec {
    fn encoded_len(&self, value: &Print) -> Result<usize, Error> {
        let size = 1usize;
        let size = size
            .checked_add(PrintArgsCodec.encoded_len(&value.print_args)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Print, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_55]).write(&(), writer)?;
        PrintArgsCodec.write(&value.print_args, writer)?;
        Ok(())
    }
}
impl Decoder<Print> for PrintCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Print, Error> {
        Constant(&[INSTRUCTION_TAG_55]).read(reader)?;
        Ok(Print {
            print_args: PrintArgsCodec.read(reader)?,
        })
    }
}
impl Print {
    fn validate_accounts(
        program: Address,
        accounts: &[AccountMeta],
        limits: Limits,
    ) -> Result<(), Error> {
        let _ = program;

        if accounts.len() != 18 {
            return Err(Error::InvalidValue);
        }

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if !accounts
            .iter()
            .zip([
                /* edition_metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), false),
                /* edition_mint */ (true, Some(false), false),
                /* edition_token_account_owner */ (false, Some(false), false),
                /* edition_token_account */ (true, Some(false), false),
                /* edition_mint_authority */ (false, Some(true), false),
                /* edition_token_record */ (true, Some(false), true),
                /* master_edition */ (true, Some(false), false),
                /* edition_marker_pda */ (true, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* master_token_account_owner */ (false, Some(true), false),
                /* master_token_account */ (false, Some(false), false),
                /* master_metadata */ (false, Some(false), false),
                /* update_authority */ (false, Some(false), false),
                /* spl_token_program */ (false, Some(false), false),
                /* spl_ata_program */ (false, Some(false), false),
                /* sysvar_instructions */ (false, Some(false), false),
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
        let data = PrintCodec.encode(self, limits.max_bytes)?;
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
        PrintCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `Resize`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resize {}
/// Bounded codec for [`Resize`].
#[derive(Clone, Copy, Debug)]
pub struct ResizeCodec;
impl Encoder<Resize> for ResizeCodec {
    fn encoded_len(&self, value: &Resize) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &Resize, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_56]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<Resize> for ResizeCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Resize, Error> {
        Constant(&[INSTRUCTION_TAG_56]).read(reader)?;
        let _ = reader;
        Ok(Resize {})
    }
}
impl Resize {
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
                /* metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), false),
                /* mint */ (false, Some(false), false),
                /* payer */ (true, Some(true), false),
                /* authority */ (false, Some(true), true),
                /* token */ (false, Some(false), true),
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
        let data = ResizeCodec.encode(self, limits.max_bytes)?;
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
        ResizeCodec.decode(&instruction.data, limits)
    }
}
/// Validated data for `CloseAccounts`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseAccounts {}
/// Bounded codec for [`CloseAccounts`].
#[derive(Clone, Copy, Debug)]
pub struct CloseAccountsCodec;
impl Encoder<CloseAccounts> for CloseAccountsCodec {
    fn encoded_len(&self, value: &CloseAccounts) -> Result<usize, Error> {
        let size = 1usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseAccounts, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[INSTRUCTION_TAG_57]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseAccounts> for CloseAccountsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseAccounts, Error> {
        Constant(&[INSTRUCTION_TAG_57]).read(reader)?;
        let _ = reader;
        Ok(CloseAccounts {})
    }
}
impl CloseAccounts {
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
                /* metadata */ (true, Some(false), false),
                /* edition */ (true, Some(false), false),
                /* mint */ (true, Some(false), false),
                /* authority */ (false, Some(true), false),
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
        let data = CloseAccountsCodec.encode(self, limits.max_bytes)?;
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
        CloseAccountsCodec.decode(&instruction.data, limits)
    }
}
/// Program error codes. Unknown numeric codes remain lossless.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MplTokenMetadataError {
    /// The `InstructionUnpackError` program error.
    InstructionUnpackError,
    /// The `InstructionPackError` program error.
    InstructionPackError,
    /// The `NotRentExempt` program error.
    NotRentExempt,
    /// The `AlreadyInitialized` program error.
    AlreadyInitialized,
    /// The `Uninitialized` program error.
    Uninitialized,
    /// The `InvalidMetadataKey` program error.
    InvalidMetadataKey,
    /// The `InvalidEditionKey` program error.
    InvalidEditionKey,
    /// The `UpdateAuthorityIncorrect` program error.
    UpdateAuthorityIncorrect,
    /// The `UpdateAuthorityIsNotSigner` program error.
    UpdateAuthorityIsNotSigner,
    /// The `NotMintAuthority` program error.
    NotMintAuthority,
    /// The `InvalidMintAuthority` program error.
    InvalidMintAuthority,
    /// The `NameTooLong` program error.
    NameTooLong,
    /// The `SymbolTooLong` program error.
    SymbolTooLong,
    /// The `UriTooLong` program error.
    UriTooLong,
    /// The `UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner` program error.
    UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner,
    /// The `MintMismatch` program error.
    MintMismatch,
    /// The `EditionsMustHaveExactlyOneToken` program error.
    EditionsMustHaveExactlyOneToken,
    /// The `MaxEditionsMintedAlready` program error.
    MaxEditionsMintedAlready,
    /// The `TokenMintToFailed` program error.
    TokenMintToFailed,
    /// The `MasterRecordMismatch` program error.
    MasterRecordMismatch,
    /// The `DestinationMintMismatch` program error.
    DestinationMintMismatch,
    /// The `EditionAlreadyMinted` program error.
    EditionAlreadyMinted,
    /// The `PrintingMintDecimalsShouldBeZero` program error.
    PrintingMintDecimalsShouldBeZero,
    /// The `OneTimePrintingAuthorizationMintDecimalsShouldBeZero` program error.
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero,
    /// The `EditionMintDecimalsShouldBeZero` program error.
    EditionMintDecimalsShouldBeZero,
    /// The `TokenBurnFailed` program error.
    TokenBurnFailed,
    /// The `TokenAccountOneTimeAuthMintMismatch` program error.
    TokenAccountOneTimeAuthMintMismatch,
    /// The `DerivedKeyInvalid` program error.
    DerivedKeyInvalid,
    /// The `PrintingMintMismatch` program error.
    PrintingMintMismatch,
    /// The `OneTimePrintingAuthMintMismatch` program error.
    OneTimePrintingAuthMintMismatch,
    /// The `TokenAccountMintMismatch` program error.
    TokenAccountMintMismatch,
    /// The `TokenAccountMintMismatchV2` program error.
    TokenAccountMintMismatchV2,
    /// The `NotEnoughTokens` program error.
    NotEnoughTokens,
    /// The `PrintingMintAuthorizationAccountMismatch` program error.
    PrintingMintAuthorizationAccountMismatch,
    /// The `AuthorizationTokenAccountOwnerMismatch` program error.
    AuthorizationTokenAccountOwnerMismatch,
    /// The `Disabled` program error.
    Disabled,
    /// The `CreatorsTooLong` program error.
    CreatorsTooLong,
    /// The `CreatorsMustBeAtleastOne` program error.
    CreatorsMustBeAtleastOne,
    /// The `MustBeOneOfCreators` program error.
    MustBeOneOfCreators,
    /// The `NoCreatorsPresentOnMetadata` program error.
    NoCreatorsPresentOnMetadata,
    /// The `CreatorNotFound` program error.
    CreatorNotFound,
    /// The `InvalidBasisPoints` program error.
    InvalidBasisPoints,
    /// The `PrimarySaleCanOnlyBeFlippedToTrue` program error.
    PrimarySaleCanOnlyBeFlippedToTrue,
    /// The `OwnerMismatch` program error.
    OwnerMismatch,
    /// The `NoBalanceInAccountForAuthorization` program error.
    NoBalanceInAccountForAuthorization,
    /// The `ShareTotalMustBe100` program error.
    ShareTotalMustBe100,
    /// The `ReservationExists` program error.
    ReservationExists,
    /// The `ReservationDoesNotExist` program error.
    ReservationDoesNotExist,
    /// The `ReservationNotSet` program error.
    ReservationNotSet,
    /// The `ReservationAlreadyMade` program error.
    ReservationAlreadyMade,
    /// The `BeyondMaxAddressSize` program error.
    BeyondMaxAddressSize,
    /// The `NumericalOverflowError` program error.
    NumericalOverflowError,
    /// The `ReservationBreachesMaximumSupply` program error.
    ReservationBreachesMaximumSupply,
    /// The `AddressNotInReservation` program error.
    AddressNotInReservation,
    /// The `CannotVerifyAnotherCreator` program error.
    CannotVerifyAnotherCreator,
    /// The `CannotUnverifyAnotherCreator` program error.
    CannotUnverifyAnotherCreator,
    /// The `SpotMismatch` program error.
    SpotMismatch,
    /// The `IncorrectOwner` program error.
    IncorrectOwner,
    /// The `PrintingWouldBreachMaximumSupply` program error.
    PrintingWouldBreachMaximumSupply,
    /// The `DataIsImmutable` program error.
    DataIsImmutable,
    /// The `DuplicateCreatorAddress` program error.
    DuplicateCreatorAddress,
    /// The `ReservationSpotsRemainingShouldMatchTotalSpotsAtStart` program error.
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,
    /// The `InvalidTokenProgram` program error.
    InvalidTokenProgram,
    /// The `DataTypeMismatch` program error.
    DataTypeMismatch,
    /// The `BeyondAlottedAddressSize` program error.
    BeyondAlottedAddressSize,
    /// The `ReservationNotComplete` program error.
    ReservationNotComplete,
    /// The `TriedToReplaceAnExistingReservation` program error.
    TriedToReplaceAnExistingReservation,
    /// The `InvalidOperation` program error.
    InvalidOperation,
    /// The `InvalidOwner` program error.
    InvalidOwner,
    /// The `PrintingMintSupplyMustBeZeroForConversion` program error.
    PrintingMintSupplyMustBeZeroForConversion,
    /// The `OneTimeAuthMintSupplyMustBeZeroForConversion` program error.
    OneTimeAuthMintSupplyMustBeZeroForConversion,
    /// The `InvalidEditionIndex` program error.
    InvalidEditionIndex,
    /// The `ReservationArrayShouldBeSizeOne` program error.
    ReservationArrayShouldBeSizeOne,
    /// The `IsMutableCanOnlyBeFlippedToFalse` program error.
    IsMutableCanOnlyBeFlippedToFalse,
    /// The `CollectionCannotBeVerifiedInThisInstruction` program error.
    CollectionCannotBeVerifiedInThisInstruction,
    /// The `Removed` program error.
    Removed,
    /// The `MustBeBurned` program error.
    MustBeBurned,
    /// The `InvalidUseMethod` program error.
    InvalidUseMethod,
    /// The `CannotChangeUseMethodAfterFirstUse` program error.
    CannotChangeUseMethodAfterFirstUse,
    /// The `CannotChangeUsesAfterFirstUse` program error.
    CannotChangeUsesAfterFirstUse,
    /// The `CollectionNotFound` program error.
    CollectionNotFound,
    /// The `InvalidCollectionUpdateAuthority` program error.
    InvalidCollectionUpdateAuthority,
    /// The `CollectionMustBeAUniqueMasterEdition` program error.
    CollectionMustBeAUniqueMasterEdition,
    /// The `UseAuthorityRecordAlreadyExists` program error.
    UseAuthorityRecordAlreadyExists,
    /// The `UseAuthorityRecordAlreadyRevoked` program error.
    UseAuthorityRecordAlreadyRevoked,
    /// The `Unusable` program error.
    Unusable,
    /// The `NotEnoughUses` program error.
    NotEnoughUses,
    /// The `CollectionAuthorityRecordAlreadyExists` program error.
    CollectionAuthorityRecordAlreadyExists,
    /// The `CollectionAuthorityDoesNotExist` program error.
    CollectionAuthorityDoesNotExist,
    /// The `InvalidUseAuthorityRecord` program error.
    InvalidUseAuthorityRecord,
    /// The `InvalidCollectionAuthorityRecord` program error.
    InvalidCollectionAuthorityRecord,
    /// The `InvalidFreezeAuthority` program error.
    InvalidFreezeAuthority,
    /// The `InvalidDelegate` program error.
    InvalidDelegate,
    /// The `CannotAdjustVerifiedCreator` program error.
    CannotAdjustVerifiedCreator,
    /// The `CannotRemoveVerifiedCreator` program error.
    CannotRemoveVerifiedCreator,
    /// The `CannotWipeVerifiedCreators` program error.
    CannotWipeVerifiedCreators,
    /// The `NotAllowedToChangeSellerFeeBasisPoints` program error.
    NotAllowedToChangeSellerFeeBasisPoints,
    /// The `EditionOverrideCannotBeZero` program error.
    EditionOverrideCannotBeZero,
    /// The `InvalidUser` program error.
    InvalidUser,
    /// The `RevokeCollectionAuthoritySignerIncorrect` program error.
    RevokeCollectionAuthoritySignerIncorrect,
    /// The `TokenCloseFailed` program error.
    TokenCloseFailed,
    /// The `UnsizedCollection` program error.
    UnsizedCollection,
    /// The `SizedCollection` program error.
    SizedCollection,
    /// The `MissingCollectionMetadata` program error.
    MissingCollectionMetadata,
    /// The `NotAMemberOfCollection` program error.
    NotAMemberOfCollection,
    /// The `NotVerifiedMemberOfCollection` program error.
    NotVerifiedMemberOfCollection,
    /// The `NotACollectionParent` program error.
    NotACollectionParent,
    /// The `CouldNotDetermineTokenStandard` program error.
    CouldNotDetermineTokenStandard,
    /// The `MissingEditionAccount` program error.
    MissingEditionAccount,
    /// The `NotAMasterEdition` program error.
    NotAMasterEdition,
    /// The `MasterEditionHasPrints` program error.
    MasterEditionHasPrints,
    /// The `BorshDeserializationError` program error.
    BorshDeserializationError,
    /// The `CannotUpdateVerifiedCollection` program error.
    CannotUpdateVerifiedCollection,
    /// The `CollectionMasterEditionAccountInvalid` program error.
    CollectionMasterEditionAccountInvalid,
    /// The `AlreadyVerified` program error.
    AlreadyVerified,
    /// The `AlreadyUnverified` program error.
    AlreadyUnverified,
    /// The `NotAPrintEdition` program error.
    NotAPrintEdition,
    /// The `InvalidMasterEdition` program error.
    InvalidMasterEdition,
    /// The `InvalidPrintEdition` program error.
    InvalidPrintEdition,
    /// The `InvalidEditionMarker` program error.
    InvalidEditionMarker,
    /// The `ReservationListDeprecated` program error.
    ReservationListDeprecated,
    /// The `PrintEditionDoesNotMatchMasterEdition` program error.
    PrintEditionDoesNotMatchMasterEdition,
    /// The `EditionNumberGreaterThanMaxSupply` program error.
    EditionNumberGreaterThanMaxSupply,
    /// The `MustUnverify` program error.
    MustUnverify,
    /// The `InvalidEscrowBumpSeed` program error.
    InvalidEscrowBumpSeed,
    /// The `MustBeEscrowAuthority` program error.
    MustBeEscrowAuthority,
    /// The `InvalidSystemProgram` program error.
    InvalidSystemProgram,
    /// The `MustBeNonFungible` program error.
    MustBeNonFungible,
    /// The `InsufficientTokens` program error.
    InsufficientTokens,
    /// The `BorshSerializationError` program error.
    BorshSerializationError,
    /// The `NoFreezeAuthoritySet` program error.
    NoFreezeAuthoritySet,
    /// The `InvalidCollectionSizeChange` program error.
    InvalidCollectionSizeChange,
    /// The `InvalidBubblegumSigner` program error.
    InvalidBubblegumSigner,
    /// The `EscrowParentHasDelegate` program error.
    EscrowParentHasDelegate,
    /// The `MintIsNotSigner` program error.
    MintIsNotSigner,
    /// The `InvalidTokenStandard` program error.
    InvalidTokenStandard,
    /// The `InvalidMintForTokenStandard` program error.
    InvalidMintForTokenStandard,
    /// The `InvalidAuthorizationRules` program error.
    InvalidAuthorizationRules,
    /// The `MissingAuthorizationRules` program error.
    MissingAuthorizationRules,
    /// The `MissingProgrammableConfig` program error.
    MissingProgrammableConfig,
    /// The `InvalidProgrammableConfig` program error.
    InvalidProgrammableConfig,
    /// The `DelegateAlreadyExists` program error.
    DelegateAlreadyExists,
    /// The `DelegateNotFound` program error.
    DelegateNotFound,
    /// The `MissingAccountInBuilder` program error.
    MissingAccountInBuilder,
    /// The `MissingArgumentInBuilder` program error.
    MissingArgumentInBuilder,
    /// The `FeatureNotSupported` program error.
    FeatureNotSupported,
    /// The `InvalidSystemWallet` program error.
    InvalidSystemWallet,
    /// The `OnlySaleDelegateCanTransfer` program error.
    OnlySaleDelegateCanTransfer,
    /// The `MissingTokenAccount` program error.
    MissingTokenAccount,
    /// The `MissingSplTokenProgram` program error.
    MissingSplTokenProgram,
    /// The `MissingAuthorizationRulesProgram` program error.
    MissingAuthorizationRulesProgram,
    /// The `InvalidDelegateRoleForTransfer` program error.
    InvalidDelegateRoleForTransfer,
    /// The `InvalidTransferAuthority` program error.
    InvalidTransferAuthority,
    /// The `InstructionNotSupported` program error.
    InstructionNotSupported,
    /// The `KeyMismatch` program error.
    KeyMismatch,
    /// The `LockedToken` program error.
    LockedToken,
    /// The `UnlockedToken` program error.
    UnlockedToken,
    /// The `MissingDelegateRole` program error.
    MissingDelegateRole,
    /// The `InvalidAuthorityType` program error.
    InvalidAuthorityType,
    /// The `MissingTokenRecord` program error.
    MissingTokenRecord,
    /// The `MintSupplyMustBeZero` program error.
    MintSupplyMustBeZero,
    /// The `DataIsEmptyOrZeroed` program error.
    DataIsEmptyOrZeroed,
    /// The `MissingTokenOwnerAccount` program error.
    MissingTokenOwnerAccount,
    /// The `InvalidMasterEditionAccountLength` program error.
    InvalidMasterEditionAccountLength,
    /// The `IncorrectTokenState` program error.
    IncorrectTokenState,
    /// The `InvalidDelegateRole` program error.
    InvalidDelegateRole,
    /// The `MissingPrintSupply` program error.
    MissingPrintSupply,
    /// The `MissingMasterEditionAccount` program error.
    MissingMasterEditionAccount,
    /// The `AmountMustBeGreaterThanZero` program error.
    AmountMustBeGreaterThanZero,
    /// The `InvalidDelegateArgs` program error.
    InvalidDelegateArgs,
    /// The `MissingLockedTransferAddress` program error.
    MissingLockedTransferAddress,
    /// The `InvalidLockedTransferAddress` program error.
    InvalidLockedTransferAddress,
    /// The `DataIncrementLimitExceeded` program error.
    DataIncrementLimitExceeded,
    /// The `CannotUpdateAssetWithDelegate` program error.
    CannotUpdateAssetWithDelegate,
    /// The `InvalidAmount` program error.
    InvalidAmount,
    /// The `MissingMasterEditionMintAccount` program error.
    MissingMasterEditionMintAccount,
    /// The `MissingMasterEditionTokenAccount` program error.
    MissingMasterEditionTokenAccount,
    /// The `MissingEditionMarkerAccount` program error.
    MissingEditionMarkerAccount,
    /// The `CannotBurnWithDelegate` program error.
    CannotBurnWithDelegate,
    /// The `MissingEdition` program error.
    MissingEdition,
    /// The `InvalidAssociatedTokenAccountProgram` program error.
    InvalidAssociatedTokenAccountProgram,
    /// The `InvalidInstructionsSysvar` program error.
    InvalidInstructionsSysvar,
    /// The `InvalidParentAccounts` program error.
    InvalidParentAccounts,
    /// The `InvalidUpdateArgs` program error.
    InvalidUpdateArgs,
    /// The `InsufficientTokenBalance` program error.
    InsufficientTokenBalance,
    /// The `MissingCollectionMint` program error.
    MissingCollectionMint,
    /// The `MissingCollectionMasterEdition` program error.
    MissingCollectionMasterEdition,
    /// The `InvalidTokenRecord` program error.
    InvalidTokenRecord,
    /// The `InvalidCloseAuthority` program error.
    InvalidCloseAuthority,
    /// The `InvalidInstruction` program error.
    InvalidInstruction,
    /// The `MissingDelegateRecord` program error.
    MissingDelegateRecord,
    /// The `InvalidFeeAccount` program error.
    InvalidFeeAccount,
    /// The `InvalidMetadataFlags` program error.
    InvalidMetadataFlags,
    /// The `CannotChangeUpdateAuthorityWithDelegate` program error.
    CannotChangeUpdateAuthorityWithDelegate,
    /// The `InvalidMintExtensionType` program error.
    InvalidMintExtensionType,
    /// The `InvalidMintCloseAuthority` program error.
    InvalidMintCloseAuthority,
    /// The `InvalidMetadataPointer` program error.
    InvalidMetadataPointer,
    /// The `InvalidTokenExtensionType` program error.
    InvalidTokenExtensionType,
    /// The `MissingImmutableOwnerExtension` program error.
    MissingImmutableOwnerExtension,
    /// The `ExpectedUninitializedAccount` program error.
    ExpectedUninitializedAccount,
    /// The `InvalidEditionAccountLength` program error.
    InvalidEditionAccountLength,
    /// The `AccountAlreadyResized` program error.
    AccountAlreadyResized,
    /// The `ConditionsForClosingNotMet` program error.
    ConditionsForClosingNotMet,
    /// An unrecognized program error code.
    Unknown(u32),
}
impl MplTokenMetadataError {
    /// Decodes known or unknown numeric program error codes.
    pub const fn from_code(code: u32) -> Self {
        match code {
            PROGRAM_ERROR_0 => Self::InstructionUnpackError,
            PROGRAM_ERROR_1 => Self::InstructionPackError,
            PROGRAM_ERROR_2 => Self::NotRentExempt,
            PROGRAM_ERROR_3 => Self::AlreadyInitialized,
            PROGRAM_ERROR_4 => Self::Uninitialized,
            PROGRAM_ERROR_5 => Self::InvalidMetadataKey,
            PROGRAM_ERROR_6 => Self::InvalidEditionKey,
            PROGRAM_ERROR_7 => Self::UpdateAuthorityIncorrect,
            PROGRAM_ERROR_8 => Self::UpdateAuthorityIsNotSigner,
            PROGRAM_ERROR_9 => Self::NotMintAuthority,
            PROGRAM_ERROR_10 => Self::InvalidMintAuthority,
            PROGRAM_ERROR_11 => Self::NameTooLong,
            PROGRAM_ERROR_12 => Self::SymbolTooLong,
            PROGRAM_ERROR_13 => Self::UriTooLong,
            PROGRAM_ERROR_14 => Self::UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner,
            PROGRAM_ERROR_15 => Self::MintMismatch,
            PROGRAM_ERROR_16 => Self::EditionsMustHaveExactlyOneToken,
            PROGRAM_ERROR_17 => Self::MaxEditionsMintedAlready,
            PROGRAM_ERROR_18 => Self::TokenMintToFailed,
            PROGRAM_ERROR_19 => Self::MasterRecordMismatch,
            PROGRAM_ERROR_20 => Self::DestinationMintMismatch,
            PROGRAM_ERROR_21 => Self::EditionAlreadyMinted,
            PROGRAM_ERROR_22 => Self::PrintingMintDecimalsShouldBeZero,
            PROGRAM_ERROR_23 => Self::OneTimePrintingAuthorizationMintDecimalsShouldBeZero,
            PROGRAM_ERROR_24 => Self::EditionMintDecimalsShouldBeZero,
            PROGRAM_ERROR_25 => Self::TokenBurnFailed,
            PROGRAM_ERROR_26 => Self::TokenAccountOneTimeAuthMintMismatch,
            PROGRAM_ERROR_27 => Self::DerivedKeyInvalid,
            PROGRAM_ERROR_28 => Self::PrintingMintMismatch,
            PROGRAM_ERROR_29 => Self::OneTimePrintingAuthMintMismatch,
            PROGRAM_ERROR_30 => Self::TokenAccountMintMismatch,
            PROGRAM_ERROR_31 => Self::TokenAccountMintMismatchV2,
            PROGRAM_ERROR_32 => Self::NotEnoughTokens,
            PROGRAM_ERROR_33 => Self::PrintingMintAuthorizationAccountMismatch,
            PROGRAM_ERROR_34 => Self::AuthorizationTokenAccountOwnerMismatch,
            PROGRAM_ERROR_35 => Self::Disabled,
            PROGRAM_ERROR_36 => Self::CreatorsTooLong,
            PROGRAM_ERROR_37 => Self::CreatorsMustBeAtleastOne,
            PROGRAM_ERROR_38 => Self::MustBeOneOfCreators,
            PROGRAM_ERROR_39 => Self::NoCreatorsPresentOnMetadata,
            PROGRAM_ERROR_40 => Self::CreatorNotFound,
            PROGRAM_ERROR_41 => Self::InvalidBasisPoints,
            PROGRAM_ERROR_42 => Self::PrimarySaleCanOnlyBeFlippedToTrue,
            PROGRAM_ERROR_43 => Self::OwnerMismatch,
            PROGRAM_ERROR_44 => Self::NoBalanceInAccountForAuthorization,
            PROGRAM_ERROR_45 => Self::ShareTotalMustBe100,
            PROGRAM_ERROR_46 => Self::ReservationExists,
            PROGRAM_ERROR_47 => Self::ReservationDoesNotExist,
            PROGRAM_ERROR_48 => Self::ReservationNotSet,
            PROGRAM_ERROR_49 => Self::ReservationAlreadyMade,
            PROGRAM_ERROR_50 => Self::BeyondMaxAddressSize,
            PROGRAM_ERROR_51 => Self::NumericalOverflowError,
            PROGRAM_ERROR_52 => Self::ReservationBreachesMaximumSupply,
            PROGRAM_ERROR_53 => Self::AddressNotInReservation,
            PROGRAM_ERROR_54 => Self::CannotVerifyAnotherCreator,
            PROGRAM_ERROR_55 => Self::CannotUnverifyAnotherCreator,
            PROGRAM_ERROR_56 => Self::SpotMismatch,
            PROGRAM_ERROR_57 => Self::IncorrectOwner,
            PROGRAM_ERROR_58 => Self::PrintingWouldBreachMaximumSupply,
            PROGRAM_ERROR_59 => Self::DataIsImmutable,
            PROGRAM_ERROR_60 => Self::DuplicateCreatorAddress,
            PROGRAM_ERROR_61 => Self::ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,
            PROGRAM_ERROR_62 => Self::InvalidTokenProgram,
            PROGRAM_ERROR_63 => Self::DataTypeMismatch,
            PROGRAM_ERROR_64 => Self::BeyondAlottedAddressSize,
            PROGRAM_ERROR_65 => Self::ReservationNotComplete,
            PROGRAM_ERROR_66 => Self::TriedToReplaceAnExistingReservation,
            PROGRAM_ERROR_67 => Self::InvalidOperation,
            PROGRAM_ERROR_68 => Self::InvalidOwner,
            PROGRAM_ERROR_69 => Self::PrintingMintSupplyMustBeZeroForConversion,
            PROGRAM_ERROR_70 => Self::OneTimeAuthMintSupplyMustBeZeroForConversion,
            PROGRAM_ERROR_71 => Self::InvalidEditionIndex,
            PROGRAM_ERROR_72 => Self::ReservationArrayShouldBeSizeOne,
            PROGRAM_ERROR_73 => Self::IsMutableCanOnlyBeFlippedToFalse,
            PROGRAM_ERROR_74 => Self::CollectionCannotBeVerifiedInThisInstruction,
            PROGRAM_ERROR_75 => Self::Removed,
            PROGRAM_ERROR_76 => Self::MustBeBurned,
            PROGRAM_ERROR_77 => Self::InvalidUseMethod,
            PROGRAM_ERROR_78 => Self::CannotChangeUseMethodAfterFirstUse,
            PROGRAM_ERROR_79 => Self::CannotChangeUsesAfterFirstUse,
            PROGRAM_ERROR_80 => Self::CollectionNotFound,
            PROGRAM_ERROR_81 => Self::InvalidCollectionUpdateAuthority,
            PROGRAM_ERROR_82 => Self::CollectionMustBeAUniqueMasterEdition,
            PROGRAM_ERROR_83 => Self::UseAuthorityRecordAlreadyExists,
            PROGRAM_ERROR_84 => Self::UseAuthorityRecordAlreadyRevoked,
            PROGRAM_ERROR_85 => Self::Unusable,
            PROGRAM_ERROR_86 => Self::NotEnoughUses,
            PROGRAM_ERROR_87 => Self::CollectionAuthorityRecordAlreadyExists,
            PROGRAM_ERROR_88 => Self::CollectionAuthorityDoesNotExist,
            PROGRAM_ERROR_89 => Self::InvalidUseAuthorityRecord,
            PROGRAM_ERROR_90 => Self::InvalidCollectionAuthorityRecord,
            PROGRAM_ERROR_91 => Self::InvalidFreezeAuthority,
            PROGRAM_ERROR_92 => Self::InvalidDelegate,
            PROGRAM_ERROR_93 => Self::CannotAdjustVerifiedCreator,
            PROGRAM_ERROR_94 => Self::CannotRemoveVerifiedCreator,
            PROGRAM_ERROR_95 => Self::CannotWipeVerifiedCreators,
            PROGRAM_ERROR_96 => Self::NotAllowedToChangeSellerFeeBasisPoints,
            PROGRAM_ERROR_97 => Self::EditionOverrideCannotBeZero,
            PROGRAM_ERROR_98 => Self::InvalidUser,
            PROGRAM_ERROR_99 => Self::RevokeCollectionAuthoritySignerIncorrect,
            PROGRAM_ERROR_100 => Self::TokenCloseFailed,
            PROGRAM_ERROR_101 => Self::UnsizedCollection,
            PROGRAM_ERROR_102 => Self::SizedCollection,
            PROGRAM_ERROR_103 => Self::MissingCollectionMetadata,
            PROGRAM_ERROR_104 => Self::NotAMemberOfCollection,
            PROGRAM_ERROR_105 => Self::NotVerifiedMemberOfCollection,
            PROGRAM_ERROR_106 => Self::NotACollectionParent,
            PROGRAM_ERROR_107 => Self::CouldNotDetermineTokenStandard,
            PROGRAM_ERROR_108 => Self::MissingEditionAccount,
            PROGRAM_ERROR_109 => Self::NotAMasterEdition,
            PROGRAM_ERROR_110 => Self::MasterEditionHasPrints,
            PROGRAM_ERROR_111 => Self::BorshDeserializationError,
            PROGRAM_ERROR_112 => Self::CannotUpdateVerifiedCollection,
            PROGRAM_ERROR_113 => Self::CollectionMasterEditionAccountInvalid,
            PROGRAM_ERROR_114 => Self::AlreadyVerified,
            PROGRAM_ERROR_115 => Self::AlreadyUnverified,
            PROGRAM_ERROR_116 => Self::NotAPrintEdition,
            PROGRAM_ERROR_117 => Self::InvalidMasterEdition,
            PROGRAM_ERROR_118 => Self::InvalidPrintEdition,
            PROGRAM_ERROR_119 => Self::InvalidEditionMarker,
            PROGRAM_ERROR_120 => Self::ReservationListDeprecated,
            PROGRAM_ERROR_121 => Self::PrintEditionDoesNotMatchMasterEdition,
            PROGRAM_ERROR_122 => Self::EditionNumberGreaterThanMaxSupply,
            PROGRAM_ERROR_123 => Self::MustUnverify,
            PROGRAM_ERROR_124 => Self::InvalidEscrowBumpSeed,
            PROGRAM_ERROR_125 => Self::MustBeEscrowAuthority,
            PROGRAM_ERROR_126 => Self::InvalidSystemProgram,
            PROGRAM_ERROR_127 => Self::MustBeNonFungible,
            PROGRAM_ERROR_128 => Self::InsufficientTokens,
            PROGRAM_ERROR_129 => Self::BorshSerializationError,
            PROGRAM_ERROR_130 => Self::NoFreezeAuthoritySet,
            PROGRAM_ERROR_131 => Self::InvalidCollectionSizeChange,
            PROGRAM_ERROR_132 => Self::InvalidBubblegumSigner,
            PROGRAM_ERROR_133 => Self::EscrowParentHasDelegate,
            PROGRAM_ERROR_134 => Self::MintIsNotSigner,
            PROGRAM_ERROR_135 => Self::InvalidTokenStandard,
            PROGRAM_ERROR_136 => Self::InvalidMintForTokenStandard,
            PROGRAM_ERROR_137 => Self::InvalidAuthorizationRules,
            PROGRAM_ERROR_138 => Self::MissingAuthorizationRules,
            PROGRAM_ERROR_139 => Self::MissingProgrammableConfig,
            PROGRAM_ERROR_140 => Self::InvalidProgrammableConfig,
            PROGRAM_ERROR_141 => Self::DelegateAlreadyExists,
            PROGRAM_ERROR_142 => Self::DelegateNotFound,
            PROGRAM_ERROR_143 => Self::MissingAccountInBuilder,
            PROGRAM_ERROR_144 => Self::MissingArgumentInBuilder,
            PROGRAM_ERROR_145 => Self::FeatureNotSupported,
            PROGRAM_ERROR_146 => Self::InvalidSystemWallet,
            PROGRAM_ERROR_147 => Self::OnlySaleDelegateCanTransfer,
            PROGRAM_ERROR_148 => Self::MissingTokenAccount,
            PROGRAM_ERROR_149 => Self::MissingSplTokenProgram,
            PROGRAM_ERROR_150 => Self::MissingAuthorizationRulesProgram,
            PROGRAM_ERROR_151 => Self::InvalidDelegateRoleForTransfer,
            PROGRAM_ERROR_152 => Self::InvalidTransferAuthority,
            PROGRAM_ERROR_153 => Self::InstructionNotSupported,
            PROGRAM_ERROR_154 => Self::KeyMismatch,
            PROGRAM_ERROR_155 => Self::LockedToken,
            PROGRAM_ERROR_156 => Self::UnlockedToken,
            PROGRAM_ERROR_157 => Self::MissingDelegateRole,
            PROGRAM_ERROR_158 => Self::InvalidAuthorityType,
            PROGRAM_ERROR_159 => Self::MissingTokenRecord,
            PROGRAM_ERROR_160 => Self::MintSupplyMustBeZero,
            PROGRAM_ERROR_161 => Self::DataIsEmptyOrZeroed,
            PROGRAM_ERROR_162 => Self::MissingTokenOwnerAccount,
            PROGRAM_ERROR_163 => Self::InvalidMasterEditionAccountLength,
            PROGRAM_ERROR_164 => Self::IncorrectTokenState,
            PROGRAM_ERROR_165 => Self::InvalidDelegateRole,
            PROGRAM_ERROR_166 => Self::MissingPrintSupply,
            PROGRAM_ERROR_167 => Self::MissingMasterEditionAccount,
            PROGRAM_ERROR_168 => Self::AmountMustBeGreaterThanZero,
            PROGRAM_ERROR_169 => Self::InvalidDelegateArgs,
            PROGRAM_ERROR_170 => Self::MissingLockedTransferAddress,
            PROGRAM_ERROR_171 => Self::InvalidLockedTransferAddress,
            PROGRAM_ERROR_172 => Self::DataIncrementLimitExceeded,
            PROGRAM_ERROR_173 => Self::CannotUpdateAssetWithDelegate,
            PROGRAM_ERROR_174 => Self::InvalidAmount,
            PROGRAM_ERROR_175 => Self::MissingMasterEditionMintAccount,
            PROGRAM_ERROR_176 => Self::MissingMasterEditionTokenAccount,
            PROGRAM_ERROR_177 => Self::MissingEditionMarkerAccount,
            PROGRAM_ERROR_178 => Self::CannotBurnWithDelegate,
            PROGRAM_ERROR_179 => Self::MissingEdition,
            PROGRAM_ERROR_180 => Self::InvalidAssociatedTokenAccountProgram,
            PROGRAM_ERROR_181 => Self::InvalidInstructionsSysvar,
            PROGRAM_ERROR_182 => Self::InvalidParentAccounts,
            PROGRAM_ERROR_183 => Self::InvalidUpdateArgs,
            PROGRAM_ERROR_184 => Self::InsufficientTokenBalance,
            PROGRAM_ERROR_185 => Self::MissingCollectionMint,
            PROGRAM_ERROR_186 => Self::MissingCollectionMasterEdition,
            PROGRAM_ERROR_187 => Self::InvalidTokenRecord,
            PROGRAM_ERROR_188 => Self::InvalidCloseAuthority,
            PROGRAM_ERROR_189 => Self::InvalidInstruction,
            PROGRAM_ERROR_190 => Self::MissingDelegateRecord,
            PROGRAM_ERROR_191 => Self::InvalidFeeAccount,
            PROGRAM_ERROR_192 => Self::InvalidMetadataFlags,
            PROGRAM_ERROR_193 => Self::CannotChangeUpdateAuthorityWithDelegate,
            PROGRAM_ERROR_194 => Self::InvalidMintExtensionType,
            PROGRAM_ERROR_195 => Self::InvalidMintCloseAuthority,
            PROGRAM_ERROR_196 => Self::InvalidMetadataPointer,
            PROGRAM_ERROR_197 => Self::InvalidTokenExtensionType,
            PROGRAM_ERROR_198 => Self::MissingImmutableOwnerExtension,
            PROGRAM_ERROR_199 => Self::ExpectedUninitializedAccount,
            PROGRAM_ERROR_200 => Self::InvalidEditionAccountLength,
            PROGRAM_ERROR_201 => Self::AccountAlreadyResized,
            PROGRAM_ERROR_202 => Self::ConditionsForClosingNotMet,
            code => Self::Unknown(code),
        }
    }
    /// Returns the exact numeric code.
    pub const fn code(self) -> u32 {
        match self {
            Self::InstructionUnpackError => PROGRAM_ERROR_0,
            Self::InstructionPackError => PROGRAM_ERROR_1,
            Self::NotRentExempt => PROGRAM_ERROR_2,
            Self::AlreadyInitialized => PROGRAM_ERROR_3,
            Self::Uninitialized => PROGRAM_ERROR_4,
            Self::InvalidMetadataKey => PROGRAM_ERROR_5,
            Self::InvalidEditionKey => PROGRAM_ERROR_6,
            Self::UpdateAuthorityIncorrect => PROGRAM_ERROR_7,
            Self::UpdateAuthorityIsNotSigner => PROGRAM_ERROR_8,
            Self::NotMintAuthority => PROGRAM_ERROR_9,
            Self::InvalidMintAuthority => PROGRAM_ERROR_10,
            Self::NameTooLong => PROGRAM_ERROR_11,
            Self::SymbolTooLong => PROGRAM_ERROR_12,
            Self::UriTooLong => PROGRAM_ERROR_13,
            Self::UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner => PROGRAM_ERROR_14,
            Self::MintMismatch => PROGRAM_ERROR_15,
            Self::EditionsMustHaveExactlyOneToken => PROGRAM_ERROR_16,
            Self::MaxEditionsMintedAlready => PROGRAM_ERROR_17,
            Self::TokenMintToFailed => PROGRAM_ERROR_18,
            Self::MasterRecordMismatch => PROGRAM_ERROR_19,
            Self::DestinationMintMismatch => PROGRAM_ERROR_20,
            Self::EditionAlreadyMinted => PROGRAM_ERROR_21,
            Self::PrintingMintDecimalsShouldBeZero => PROGRAM_ERROR_22,
            Self::OneTimePrintingAuthorizationMintDecimalsShouldBeZero => PROGRAM_ERROR_23,
            Self::EditionMintDecimalsShouldBeZero => PROGRAM_ERROR_24,
            Self::TokenBurnFailed => PROGRAM_ERROR_25,
            Self::TokenAccountOneTimeAuthMintMismatch => PROGRAM_ERROR_26,
            Self::DerivedKeyInvalid => PROGRAM_ERROR_27,
            Self::PrintingMintMismatch => PROGRAM_ERROR_28,
            Self::OneTimePrintingAuthMintMismatch => PROGRAM_ERROR_29,
            Self::TokenAccountMintMismatch => PROGRAM_ERROR_30,
            Self::TokenAccountMintMismatchV2 => PROGRAM_ERROR_31,
            Self::NotEnoughTokens => PROGRAM_ERROR_32,
            Self::PrintingMintAuthorizationAccountMismatch => PROGRAM_ERROR_33,
            Self::AuthorizationTokenAccountOwnerMismatch => PROGRAM_ERROR_34,
            Self::Disabled => PROGRAM_ERROR_35,
            Self::CreatorsTooLong => PROGRAM_ERROR_36,
            Self::CreatorsMustBeAtleastOne => PROGRAM_ERROR_37,
            Self::MustBeOneOfCreators => PROGRAM_ERROR_38,
            Self::NoCreatorsPresentOnMetadata => PROGRAM_ERROR_39,
            Self::CreatorNotFound => PROGRAM_ERROR_40,
            Self::InvalidBasisPoints => PROGRAM_ERROR_41,
            Self::PrimarySaleCanOnlyBeFlippedToTrue => PROGRAM_ERROR_42,
            Self::OwnerMismatch => PROGRAM_ERROR_43,
            Self::NoBalanceInAccountForAuthorization => PROGRAM_ERROR_44,
            Self::ShareTotalMustBe100 => PROGRAM_ERROR_45,
            Self::ReservationExists => PROGRAM_ERROR_46,
            Self::ReservationDoesNotExist => PROGRAM_ERROR_47,
            Self::ReservationNotSet => PROGRAM_ERROR_48,
            Self::ReservationAlreadyMade => PROGRAM_ERROR_49,
            Self::BeyondMaxAddressSize => PROGRAM_ERROR_50,
            Self::NumericalOverflowError => PROGRAM_ERROR_51,
            Self::ReservationBreachesMaximumSupply => PROGRAM_ERROR_52,
            Self::AddressNotInReservation => PROGRAM_ERROR_53,
            Self::CannotVerifyAnotherCreator => PROGRAM_ERROR_54,
            Self::CannotUnverifyAnotherCreator => PROGRAM_ERROR_55,
            Self::SpotMismatch => PROGRAM_ERROR_56,
            Self::IncorrectOwner => PROGRAM_ERROR_57,
            Self::PrintingWouldBreachMaximumSupply => PROGRAM_ERROR_58,
            Self::DataIsImmutable => PROGRAM_ERROR_59,
            Self::DuplicateCreatorAddress => PROGRAM_ERROR_60,
            Self::ReservationSpotsRemainingShouldMatchTotalSpotsAtStart => PROGRAM_ERROR_61,
            Self::InvalidTokenProgram => PROGRAM_ERROR_62,
            Self::DataTypeMismatch => PROGRAM_ERROR_63,
            Self::BeyondAlottedAddressSize => PROGRAM_ERROR_64,
            Self::ReservationNotComplete => PROGRAM_ERROR_65,
            Self::TriedToReplaceAnExistingReservation => PROGRAM_ERROR_66,
            Self::InvalidOperation => PROGRAM_ERROR_67,
            Self::InvalidOwner => PROGRAM_ERROR_68,
            Self::PrintingMintSupplyMustBeZeroForConversion => PROGRAM_ERROR_69,
            Self::OneTimeAuthMintSupplyMustBeZeroForConversion => PROGRAM_ERROR_70,
            Self::InvalidEditionIndex => PROGRAM_ERROR_71,
            Self::ReservationArrayShouldBeSizeOne => PROGRAM_ERROR_72,
            Self::IsMutableCanOnlyBeFlippedToFalse => PROGRAM_ERROR_73,
            Self::CollectionCannotBeVerifiedInThisInstruction => PROGRAM_ERROR_74,
            Self::Removed => PROGRAM_ERROR_75,
            Self::MustBeBurned => PROGRAM_ERROR_76,
            Self::InvalidUseMethod => PROGRAM_ERROR_77,
            Self::CannotChangeUseMethodAfterFirstUse => PROGRAM_ERROR_78,
            Self::CannotChangeUsesAfterFirstUse => PROGRAM_ERROR_79,
            Self::CollectionNotFound => PROGRAM_ERROR_80,
            Self::InvalidCollectionUpdateAuthority => PROGRAM_ERROR_81,
            Self::CollectionMustBeAUniqueMasterEdition => PROGRAM_ERROR_82,
            Self::UseAuthorityRecordAlreadyExists => PROGRAM_ERROR_83,
            Self::UseAuthorityRecordAlreadyRevoked => PROGRAM_ERROR_84,
            Self::Unusable => PROGRAM_ERROR_85,
            Self::NotEnoughUses => PROGRAM_ERROR_86,
            Self::CollectionAuthorityRecordAlreadyExists => PROGRAM_ERROR_87,
            Self::CollectionAuthorityDoesNotExist => PROGRAM_ERROR_88,
            Self::InvalidUseAuthorityRecord => PROGRAM_ERROR_89,
            Self::InvalidCollectionAuthorityRecord => PROGRAM_ERROR_90,
            Self::InvalidFreezeAuthority => PROGRAM_ERROR_91,
            Self::InvalidDelegate => PROGRAM_ERROR_92,
            Self::CannotAdjustVerifiedCreator => PROGRAM_ERROR_93,
            Self::CannotRemoveVerifiedCreator => PROGRAM_ERROR_94,
            Self::CannotWipeVerifiedCreators => PROGRAM_ERROR_95,
            Self::NotAllowedToChangeSellerFeeBasisPoints => PROGRAM_ERROR_96,
            Self::EditionOverrideCannotBeZero => PROGRAM_ERROR_97,
            Self::InvalidUser => PROGRAM_ERROR_98,
            Self::RevokeCollectionAuthoritySignerIncorrect => PROGRAM_ERROR_99,
            Self::TokenCloseFailed => PROGRAM_ERROR_100,
            Self::UnsizedCollection => PROGRAM_ERROR_101,
            Self::SizedCollection => PROGRAM_ERROR_102,
            Self::MissingCollectionMetadata => PROGRAM_ERROR_103,
            Self::NotAMemberOfCollection => PROGRAM_ERROR_104,
            Self::NotVerifiedMemberOfCollection => PROGRAM_ERROR_105,
            Self::NotACollectionParent => PROGRAM_ERROR_106,
            Self::CouldNotDetermineTokenStandard => PROGRAM_ERROR_107,
            Self::MissingEditionAccount => PROGRAM_ERROR_108,
            Self::NotAMasterEdition => PROGRAM_ERROR_109,
            Self::MasterEditionHasPrints => PROGRAM_ERROR_110,
            Self::BorshDeserializationError => PROGRAM_ERROR_111,
            Self::CannotUpdateVerifiedCollection => PROGRAM_ERROR_112,
            Self::CollectionMasterEditionAccountInvalid => PROGRAM_ERROR_113,
            Self::AlreadyVerified => PROGRAM_ERROR_114,
            Self::AlreadyUnverified => PROGRAM_ERROR_115,
            Self::NotAPrintEdition => PROGRAM_ERROR_116,
            Self::InvalidMasterEdition => PROGRAM_ERROR_117,
            Self::InvalidPrintEdition => PROGRAM_ERROR_118,
            Self::InvalidEditionMarker => PROGRAM_ERROR_119,
            Self::ReservationListDeprecated => PROGRAM_ERROR_120,
            Self::PrintEditionDoesNotMatchMasterEdition => PROGRAM_ERROR_121,
            Self::EditionNumberGreaterThanMaxSupply => PROGRAM_ERROR_122,
            Self::MustUnverify => PROGRAM_ERROR_123,
            Self::InvalidEscrowBumpSeed => PROGRAM_ERROR_124,
            Self::MustBeEscrowAuthority => PROGRAM_ERROR_125,
            Self::InvalidSystemProgram => PROGRAM_ERROR_126,
            Self::MustBeNonFungible => PROGRAM_ERROR_127,
            Self::InsufficientTokens => PROGRAM_ERROR_128,
            Self::BorshSerializationError => PROGRAM_ERROR_129,
            Self::NoFreezeAuthoritySet => PROGRAM_ERROR_130,
            Self::InvalidCollectionSizeChange => PROGRAM_ERROR_131,
            Self::InvalidBubblegumSigner => PROGRAM_ERROR_132,
            Self::EscrowParentHasDelegate => PROGRAM_ERROR_133,
            Self::MintIsNotSigner => PROGRAM_ERROR_134,
            Self::InvalidTokenStandard => PROGRAM_ERROR_135,
            Self::InvalidMintForTokenStandard => PROGRAM_ERROR_136,
            Self::InvalidAuthorizationRules => PROGRAM_ERROR_137,
            Self::MissingAuthorizationRules => PROGRAM_ERROR_138,
            Self::MissingProgrammableConfig => PROGRAM_ERROR_139,
            Self::InvalidProgrammableConfig => PROGRAM_ERROR_140,
            Self::DelegateAlreadyExists => PROGRAM_ERROR_141,
            Self::DelegateNotFound => PROGRAM_ERROR_142,
            Self::MissingAccountInBuilder => PROGRAM_ERROR_143,
            Self::MissingArgumentInBuilder => PROGRAM_ERROR_144,
            Self::FeatureNotSupported => PROGRAM_ERROR_145,
            Self::InvalidSystemWallet => PROGRAM_ERROR_146,
            Self::OnlySaleDelegateCanTransfer => PROGRAM_ERROR_147,
            Self::MissingTokenAccount => PROGRAM_ERROR_148,
            Self::MissingSplTokenProgram => PROGRAM_ERROR_149,
            Self::MissingAuthorizationRulesProgram => PROGRAM_ERROR_150,
            Self::InvalidDelegateRoleForTransfer => PROGRAM_ERROR_151,
            Self::InvalidTransferAuthority => PROGRAM_ERROR_152,
            Self::InstructionNotSupported => PROGRAM_ERROR_153,
            Self::KeyMismatch => PROGRAM_ERROR_154,
            Self::LockedToken => PROGRAM_ERROR_155,
            Self::UnlockedToken => PROGRAM_ERROR_156,
            Self::MissingDelegateRole => PROGRAM_ERROR_157,
            Self::InvalidAuthorityType => PROGRAM_ERROR_158,
            Self::MissingTokenRecord => PROGRAM_ERROR_159,
            Self::MintSupplyMustBeZero => PROGRAM_ERROR_160,
            Self::DataIsEmptyOrZeroed => PROGRAM_ERROR_161,
            Self::MissingTokenOwnerAccount => PROGRAM_ERROR_162,
            Self::InvalidMasterEditionAccountLength => PROGRAM_ERROR_163,
            Self::IncorrectTokenState => PROGRAM_ERROR_164,
            Self::InvalidDelegateRole => PROGRAM_ERROR_165,
            Self::MissingPrintSupply => PROGRAM_ERROR_166,
            Self::MissingMasterEditionAccount => PROGRAM_ERROR_167,
            Self::AmountMustBeGreaterThanZero => PROGRAM_ERROR_168,
            Self::InvalidDelegateArgs => PROGRAM_ERROR_169,
            Self::MissingLockedTransferAddress => PROGRAM_ERROR_170,
            Self::InvalidLockedTransferAddress => PROGRAM_ERROR_171,
            Self::DataIncrementLimitExceeded => PROGRAM_ERROR_172,
            Self::CannotUpdateAssetWithDelegate => PROGRAM_ERROR_173,
            Self::InvalidAmount => PROGRAM_ERROR_174,
            Self::MissingMasterEditionMintAccount => PROGRAM_ERROR_175,
            Self::MissingMasterEditionTokenAccount => PROGRAM_ERROR_176,
            Self::MissingEditionMarkerAccount => PROGRAM_ERROR_177,
            Self::CannotBurnWithDelegate => PROGRAM_ERROR_178,
            Self::MissingEdition => PROGRAM_ERROR_179,
            Self::InvalidAssociatedTokenAccountProgram => PROGRAM_ERROR_180,
            Self::InvalidInstructionsSysvar => PROGRAM_ERROR_181,
            Self::InvalidParentAccounts => PROGRAM_ERROR_182,
            Self::InvalidUpdateArgs => PROGRAM_ERROR_183,
            Self::InsufficientTokenBalance => PROGRAM_ERROR_184,
            Self::MissingCollectionMint => PROGRAM_ERROR_185,
            Self::MissingCollectionMasterEdition => PROGRAM_ERROR_186,
            Self::InvalidTokenRecord => PROGRAM_ERROR_187,
            Self::InvalidCloseAuthority => PROGRAM_ERROR_188,
            Self::InvalidInstruction => PROGRAM_ERROR_189,
            Self::MissingDelegateRecord => PROGRAM_ERROR_190,
            Self::InvalidFeeAccount => PROGRAM_ERROR_191,
            Self::InvalidMetadataFlags => PROGRAM_ERROR_192,
            Self::CannotChangeUpdateAuthorityWithDelegate => PROGRAM_ERROR_193,
            Self::InvalidMintExtensionType => PROGRAM_ERROR_194,
            Self::InvalidMintCloseAuthority => PROGRAM_ERROR_195,
            Self::InvalidMetadataPointer => PROGRAM_ERROR_196,
            Self::InvalidTokenExtensionType => PROGRAM_ERROR_197,
            Self::MissingImmutableOwnerExtension => PROGRAM_ERROR_198,
            Self::ExpectedUninitializedAccount => PROGRAM_ERROR_199,
            Self::InvalidEditionAccountLength => PROGRAM_ERROR_200,
            Self::AccountAlreadyResized => PROGRAM_ERROR_201,
            Self::ConditionsForClosingNotMet => PROGRAM_ERROR_202,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the source message for known errors.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::InstructionUnpackError => Some(""),
            Self::InstructionPackError => Some(""),
            Self::NotRentExempt => Some("Lamport balance below rent-exempt threshold"),
            Self::AlreadyInitialized => Some("Already initialized"),
            Self::Uninitialized => Some("Uninitialized"),
            Self::InvalidMetadataKey => {
                Some(" Metadata's key must match seed of ['metadata', program id, mint] provided")
            }
            Self::InvalidEditionKey => Some(
                "Edition's key must match seed of ['metadata', program id, name, 'edition'] provided",
            ),
            Self::UpdateAuthorityIncorrect => Some("Update Authority given does not match"),
            Self::UpdateAuthorityIsNotSigner => {
                Some("Update Authority needs to be signer to update metadata")
            }
            Self::NotMintAuthority => {
                Some("You must be the mint authority and signer on this transaction")
            }
            Self::InvalidMintAuthority => {
                Some("Mint authority provided does not match the authority on the mint")
            }
            Self::NameTooLong => Some("Name too long"),
            Self::SymbolTooLong => Some("Symbol too long"),
            Self::UriTooLong => Some("URI too long"),
            Self::UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner => Some(""),
            Self::MintMismatch => Some("Mint given does not match mint on Metadata"),
            Self::EditionsMustHaveExactlyOneToken => Some("Editions must have exactly one token"),
            Self::MaxEditionsMintedAlready => Some(""),
            Self::TokenMintToFailed => Some(""),
            Self::MasterRecordMismatch => Some(""),
            Self::DestinationMintMismatch => Some(""),
            Self::EditionAlreadyMinted => Some(""),
            Self::PrintingMintDecimalsShouldBeZero => Some(""),
            Self::OneTimePrintingAuthorizationMintDecimalsShouldBeZero => Some(""),
            Self::EditionMintDecimalsShouldBeZero => Some("EditionMintDecimalsShouldBeZero"),
            Self::TokenBurnFailed => Some(""),
            Self::TokenAccountOneTimeAuthMintMismatch => Some(""),
            Self::DerivedKeyInvalid => Some("Derived key invalid"),
            Self::PrintingMintMismatch => {
                Some("The Printing mint does not match that on the master edition!")
            }
            Self::OneTimePrintingAuthMintMismatch => {
                Some("The One Time Printing Auth mint does not match that on the master edition!")
            }
            Self::TokenAccountMintMismatch => {
                Some("The mint of the token account does not match the Printing mint!")
            }
            Self::TokenAccountMintMismatchV2 => {
                Some("The mint of the token account does not match the master metadata mint!")
            }
            Self::NotEnoughTokens => Some("Not enough tokens to mint a limited edition"),
            Self::PrintingMintAuthorizationAccountMismatch => Some(""),
            Self::AuthorizationTokenAccountOwnerMismatch => Some(""),
            Self::Disabled => Some(""),
            Self::CreatorsTooLong => Some("Creators list too long"),
            Self::CreatorsMustBeAtleastOne => Some("Creators must be at least one if set"),
            Self::MustBeOneOfCreators => Some(""),
            Self::NoCreatorsPresentOnMetadata => Some("This metadata does not have creators"),
            Self::CreatorNotFound => Some("This creator address was not found"),
            Self::InvalidBasisPoints => Some("Basis points cannot be more than 10000"),
            Self::PrimarySaleCanOnlyBeFlippedToTrue => {
                Some("Primary sale can only be flipped to true and is immutable")
            }
            Self::OwnerMismatch => Some("Owner does not match that on the account given"),
            Self::NoBalanceInAccountForAuthorization => {
                Some("This account has no tokens to be used for authorization")
            }
            Self::ShareTotalMustBe100 => Some("Share total must equal 100 for creator array"),
            Self::ReservationExists => Some(""),
            Self::ReservationDoesNotExist => Some(""),
            Self::ReservationNotSet => Some(""),
            Self::ReservationAlreadyMade => Some(""),
            Self::BeyondMaxAddressSize => Some(""),
            Self::NumericalOverflowError => Some("NumericalOverflowError"),
            Self::ReservationBreachesMaximumSupply => Some(""),
            Self::AddressNotInReservation => Some(""),
            Self::CannotVerifyAnotherCreator => {
                Some("You cannot unilaterally verify another creator, they must sign")
            }
            Self::CannotUnverifyAnotherCreator => {
                Some("You cannot unilaterally unverify another creator")
            }
            Self::SpotMismatch => Some(""),
            Self::IncorrectOwner => Some("Incorrect account owner"),
            Self::PrintingWouldBreachMaximumSupply => Some(""),
            Self::DataIsImmutable => Some("Data is immutable"),
            Self::DuplicateCreatorAddress => Some("No duplicate creator addresses"),
            Self::ReservationSpotsRemainingShouldMatchTotalSpotsAtStart => Some(""),
            Self::InvalidTokenProgram => Some("Invalid token program"),
            Self::DataTypeMismatch => Some("Data type mismatch"),
            Self::BeyondAlottedAddressSize => Some(""),
            Self::ReservationNotComplete => Some(""),
            Self::TriedToReplaceAnExistingReservation => Some(""),
            Self::InvalidOperation => Some("Invalid operation"),
            Self::InvalidOwner => Some("Invalid Owner"),
            Self::PrintingMintSupplyMustBeZeroForConversion => {
                Some("Printing mint supply must be zero for conversion")
            }
            Self::OneTimeAuthMintSupplyMustBeZeroForConversion => {
                Some("One Time Auth mint supply must be zero for conversion")
            }
            Self::InvalidEditionIndex => {
                Some("You tried to insert one edition too many into an edition mark pda")
            }
            Self::ReservationArrayShouldBeSizeOne => Some(""),
            Self::IsMutableCanOnlyBeFlippedToFalse => {
                Some("Is Mutable can only be flipped to false")
            }
            Self::CollectionCannotBeVerifiedInThisInstruction => {
                Some("Collection cannot be verified in this instruction")
            }
            Self::Removed => {
                Some("This instruction was deprecated in a previous release and is now removed")
            }
            Self::MustBeBurned => Some(""),
            Self::InvalidUseMethod => Some("This use method is invalid"),
            Self::CannotChangeUseMethodAfterFirstUse => {
                Some("Cannot Change Use Method after the first use")
            }
            Self::CannotChangeUsesAfterFirstUse => {
                Some("Cannot Change Remaining or Available uses after the first use")
            }
            Self::CollectionNotFound => Some("Collection Not Found on Metadata"),
            Self::InvalidCollectionUpdateAuthority => {
                Some("Collection Update Authority is invalid")
            }
            Self::CollectionMustBeAUniqueMasterEdition => {
                Some("Collection Must Be a Unique Master Edition v2")
            }
            Self::UseAuthorityRecordAlreadyExists => {
                Some("The Use Authority Record Already Exists, to modify it Revoke, then Approve")
            }
            Self::UseAuthorityRecordAlreadyRevoked => {
                Some("The Use Authority Record is empty or already revoked")
            }
            Self::Unusable => Some("This token has no uses"),
            Self::NotEnoughUses => Some("There are not enough Uses left on this token."),
            Self::CollectionAuthorityRecordAlreadyExists => {
                Some("This Collection Authority Record Already Exists.")
            }
            Self::CollectionAuthorityDoesNotExist => {
                Some("This Collection Authority Record Does Not Exist.")
            }
            Self::InvalidUseAuthorityRecord => Some("This Use Authority Record is invalid."),
            Self::InvalidCollectionAuthorityRecord => Some(""),
            Self::InvalidFreezeAuthority => {
                Some("Metadata does not match the freeze authority on the mint")
            }
            Self::InvalidDelegate => {
                Some("All tokens in this account have not been delegated to this user.")
            }
            Self::CannotAdjustVerifiedCreator => Some(""),
            Self::CannotRemoveVerifiedCreator => Some("Verified creators cannot be removed."),
            Self::CannotWipeVerifiedCreators => Some(""),
            Self::NotAllowedToChangeSellerFeeBasisPoints => Some(""),
            Self::EditionOverrideCannotBeZero => Some("Edition override cannot be zero"),
            Self::InvalidUser => Some("Invalid User"),
            Self::RevokeCollectionAuthoritySignerIncorrect => {
                Some("Revoke Collection Authority signer is incorrect")
            }
            Self::TokenCloseFailed => Some(""),
            Self::UnsizedCollection => Some("Can't use this function on unsized collection"),
            Self::SizedCollection => Some("Can't use this function on a sized collection"),
            Self::MissingCollectionMetadata => Some("Missing collection metadata account"),
            Self::NotAMemberOfCollection => {
                Some("This NFT is not a member of the specified collection.")
            }
            Self::NotVerifiedMemberOfCollection => {
                Some("This NFT is not a verified member of the specified collection.")
            }
            Self::NotACollectionParent => Some("This NFT is not a collection parent NFT."),
            Self::CouldNotDetermineTokenStandard => {
                Some("Could not determine a TokenStandard type.")
            }
            Self::MissingEditionAccount => {
                Some("This mint account has an edition but none was provided.")
            }
            Self::NotAMasterEdition => Some("This edition is not a Master Edition"),
            Self::MasterEditionHasPrints => Some("This Master Edition has existing prints"),
            Self::BorshDeserializationError => Some(""),
            Self::CannotUpdateVerifiedCollection => {
                Some("Cannot update a verified collection in this command")
            }
            Self::CollectionMasterEditionAccountInvalid => {
                Some("Edition account doesnt match collection ")
            }
            Self::AlreadyVerified => Some("Item is already verified."),
            Self::AlreadyUnverified => Some(""),
            Self::NotAPrintEdition => Some("This edition is not a Print Edition"),
            Self::InvalidMasterEdition => Some("Invalid Master Edition"),
            Self::InvalidPrintEdition => Some("Invalid Print Edition"),
            Self::InvalidEditionMarker => Some("Invalid Edition Marker"),
            Self::ReservationListDeprecated => Some("Reservation List is Deprecated"),
            Self::PrintEditionDoesNotMatchMasterEdition => {
                Some("Print Edition does not match Master Edition")
            }
            Self::EditionNumberGreaterThanMaxSupply => {
                Some("Edition Number greater than max supply")
            }
            Self::MustUnverify => Some("Must unverify before migrating collections."),
            Self::InvalidEscrowBumpSeed => Some("Invalid Escrow Account Bump Seed"),
            Self::MustBeEscrowAuthority => Some("Must Escrow Authority"),
            Self::InvalidSystemProgram => Some("Invalid System Program"),
            Self::MustBeNonFungible => Some("Must be a Non Fungible Token"),
            Self::InsufficientTokens => Some("Insufficient tokens for transfer"),
            Self::BorshSerializationError => Some("Borsh Serialization Error"),
            Self::NoFreezeAuthoritySet => Some("Cannot create NFT with no Freeze Authority."),
            Self::InvalidCollectionSizeChange => Some("Invalid collection size change"),
            Self::InvalidBubblegumSigner => Some("Invalid bubblegum signer"),
            Self::EscrowParentHasDelegate => Some("Escrow parent cannot have a delegate"),
            Self::MintIsNotSigner => Some("Mint needs to be signer to initialize the account"),
            Self::InvalidTokenStandard => Some("Invalid token standard"),
            Self::InvalidMintForTokenStandard => {
                Some("Invalid mint account for specified token standard")
            }
            Self::InvalidAuthorizationRules => Some("Invalid authorization rules account"),
            Self::MissingAuthorizationRules => Some("Missing authorization rules account"),
            Self::MissingProgrammableConfig => Some("Missing programmable configuration"),
            Self::InvalidProgrammableConfig => Some("Invalid programmable configuration"),
            Self::DelegateAlreadyExists => Some("Delegate already exists"),
            Self::DelegateNotFound => Some("Delegate not found"),
            Self::MissingAccountInBuilder => {
                Some("Required account not set in instruction builder")
            }
            Self::MissingArgumentInBuilder => {
                Some("Required argument not set in instruction builder")
            }
            Self::FeatureNotSupported => Some("Feature not supported currently"),
            Self::InvalidSystemWallet => Some("Invalid system wallet"),
            Self::OnlySaleDelegateCanTransfer => {
                Some("Only the sale delegate can transfer while its set")
            }
            Self::MissingTokenAccount => Some("Missing token account"),
            Self::MissingSplTokenProgram => Some("Missing SPL token program"),
            Self::MissingAuthorizationRulesProgram => Some("Missing authorization rules program"),
            Self::InvalidDelegateRoleForTransfer => Some("Invalid delegate role for transfer"),
            Self::InvalidTransferAuthority => Some("Invalid transfer authority"),
            Self::InstructionNotSupported => {
                Some("Instruction not supported for ProgrammableNonFungible assets")
            }
            Self::KeyMismatch => Some("Public key does not match expected value"),
            Self::LockedToken => Some("Token is locked"),
            Self::UnlockedToken => Some("Token is unlocked"),
            Self::MissingDelegateRole => Some("Missing delegate role"),
            Self::InvalidAuthorityType => Some("Invalid authority type"),
            Self::MissingTokenRecord => Some("Missing token record account"),
            Self::MintSupplyMustBeZero => Some("Mint supply must be zero for programmable assets"),
            Self::DataIsEmptyOrZeroed => Some("Data is empty or zeroed"),
            Self::MissingTokenOwnerAccount => Some("Missing token owner"),
            Self::InvalidMasterEditionAccountLength => {
                Some("Master edition account has an invalid length")
            }
            Self::IncorrectTokenState => Some("Incorrect token state"),
            Self::InvalidDelegateRole => Some("Invalid delegate role"),
            Self::MissingPrintSupply => Some("Print supply is required for non-fungibles"),
            Self::MissingMasterEditionAccount => Some("Missing master edition account"),
            Self::AmountMustBeGreaterThanZero => Some("Amount must be greater than zero"),
            Self::InvalidDelegateArgs => Some("Invalid delegate args"),
            Self::MissingLockedTransferAddress => Some("Missing address for locked transfer"),
            Self::InvalidLockedTransferAddress => {
                Some("Invalid destination address for locked transfer")
            }
            Self::DataIncrementLimitExceeded => Some("Exceeded account realloc increase limit"),
            Self::CannotUpdateAssetWithDelegate => {
                Some("Cannot update the rule set of a programmable asset that has a delegate")
            }
            Self::InvalidAmount => {
                Some("Invalid token amount for this operation or token standard")
            }
            Self::MissingMasterEditionMintAccount => Some("Missing master edition mint account"),
            Self::MissingMasterEditionTokenAccount => Some("Missing master edition token account"),
            Self::MissingEditionMarkerAccount => Some("Missing edition marker account"),
            Self::CannotBurnWithDelegate => Some("Cannot burn while persistent delegate is set"),
            Self::MissingEdition => Some("Missing edition account"),
            Self::InvalidAssociatedTokenAccountProgram => {
                Some("Invalid Associated Token Account Program")
            }
            Self::InvalidInstructionsSysvar => Some("Invalid InstructionsSysvar"),
            Self::InvalidParentAccounts => Some("Invalid or Unneeded parent accounts"),
            Self::InvalidUpdateArgs => Some("Authority cannot apply all update args"),
            Self::InsufficientTokenBalance => Some("Token account does not have enough tokens"),
            Self::MissingCollectionMint => Some("Missing collection account"),
            Self::MissingCollectionMasterEdition => {
                Some("Missing collection master edition account")
            }
            Self::InvalidTokenRecord => Some("Invalid token record account"),
            Self::InvalidCloseAuthority => {
                Some("The close authority needs to be revoked by the Utility Delegate")
            }
            Self::InvalidInstruction => Some("Invalid or removed instruction"),
            Self::MissingDelegateRecord => Some("Missing delegate record"),
            Self::InvalidFeeAccount => Some(""),
            Self::InvalidMetadataFlags => Some(""),
            Self::CannotChangeUpdateAuthorityWithDelegate => {
                Some("Cannot change the update authority with a delegate")
            }
            Self::InvalidMintExtensionType => Some("Invalid mint extension type"),
            Self::InvalidMintCloseAuthority => Some("Invalid mint close authority"),
            Self::InvalidMetadataPointer => Some("Invalid metadata pointer"),
            Self::InvalidTokenExtensionType => Some("Invalid token extension type"),
            Self::MissingImmutableOwnerExtension => Some("Missing immutable owner extension"),
            Self::ExpectedUninitializedAccount => Some("Expected account to be uninitialized"),
            Self::InvalidEditionAccountLength => Some("Edition account has an invalid length"),
            Self::AccountAlreadyResized => Some("Account has already been resized"),
            Self::ConditionsForClosingNotMet => Some("Conditions for closing not met"),
            Self::Unknown(_) => None,
        }
    }
}
/// All selected instruction data variants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MplTokenMetadataInstructionData {
    /// Data for tag 0.
    CreateMetadataAccount(CreateMetadataAccount),
    /// Data for tag 1.
    UpdateMetadataAccount(UpdateMetadataAccount),
    /// Data for tag 2.
    DeprecatedCreateMasterEdition(DeprecatedCreateMasterEdition),
    /// Data for tag 3.
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(
        DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
    ),
    /// Data for tag 4.
    UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaToken),
    /// Data for tag 5.
    DeprecatedSetReservationList(DeprecatedSetReservationList),
    /// Data for tag 6.
    DeprecatedCreateReservationList(DeprecatedCreateReservationList),
    /// Data for tag 7.
    SignMetadata(SignMetadata),
    /// Data for tag 8.
    DeprecatedMintPrintingTokensViaToken(DeprecatedMintPrintingTokensViaToken),
    /// Data for tag 9.
    DeprecatedMintPrintingTokens(DeprecatedMintPrintingTokens),
    /// Data for tag 10.
    CreateMasterEdition(CreateMasterEdition),
    /// Data for tag 11.
    MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaToken),
    /// Data for tag 12.
    ConvertMasterEditionV1ToV2(ConvertMasterEditionV1ToV2),
    /// Data for tag 13.
    MintNewEditionFromMasterEditionViaVaultProxy(MintNewEditionFromMasterEditionViaVaultProxy),
    /// Data for tag 14.
    PuffMetadata(PuffMetadata),
    /// Data for tag 15.
    UpdateMetadataAccountV2(UpdateMetadataAccountV2),
    /// Data for tag 16.
    CreateMetadataAccountV2(CreateMetadataAccountV2),
    /// Data for tag 17.
    CreateMasterEditionV3(CreateMasterEditionV3),
    /// Data for tag 18.
    VerifyCollection(VerifyCollection),
    /// Data for tag 19.
    Utilize(Utilize),
    /// Data for tag 20.
    ApproveUseAuthority(ApproveUseAuthority),
    /// Data for tag 21.
    RevokeUseAuthority(RevokeUseAuthority),
    /// Data for tag 22.
    UnverifyCollection(UnverifyCollection),
    /// Data for tag 23.
    ApproveCollectionAuthority(ApproveCollectionAuthority),
    /// Data for tag 24.
    RevokeCollectionAuthority(RevokeCollectionAuthority),
    /// Data for tag 25.
    SetAndVerifyCollection(SetAndVerifyCollection),
    /// Data for tag 26.
    FreezeDelegatedAccount(FreezeDelegatedAccount),
    /// Data for tag 27.
    ThawDelegatedAccount(ThawDelegatedAccount),
    /// Data for tag 28.
    RemoveCreatorVerification(RemoveCreatorVerification),
    /// Data for tag 29.
    BurnNft(BurnNft),
    /// Data for tag 30.
    VerifySizedCollectionItem(VerifySizedCollectionItem),
    /// Data for tag 31.
    UnverifySizedCollectionItem(UnverifySizedCollectionItem),
    /// Data for tag 32.
    SetAndVerifySizedCollectionItem(SetAndVerifySizedCollectionItem),
    /// Data for tag 33.
    CreateMetadataAccountV3(CreateMetadataAccountV3),
    /// Data for tag 34.
    SetCollectionSize(SetCollectionSize),
    /// Data for tag 35.
    SetTokenStandard(SetTokenStandard),
    /// Data for tag 36.
    BubblegumSetCollectionSize(BubblegumSetCollectionSize),
    /// Data for tag 37.
    BurnEditionNft(BurnEditionNft),
    /// Data for tag 38.
    CreateEscrowAccount(CreateEscrowAccount),
    /// Data for tag 39.
    CloseEscrowAccount(CloseEscrowAccount),
    /// Data for tag 40.
    TransferOutOfEscrow(TransferOutOfEscrow),
    /// Data for tag 41.
    Burn(Burn),
    /// Data for tag 42.
    Create(Create),
    /// Data for tag 43.
    Mint(Mint),
    /// Data for tag 44.
    Delegate(Delegate),
    /// Data for tag 45.
    Revoke(Revoke),
    /// Data for tag 46.
    Lock(Lock),
    /// Data for tag 47.
    Unlock(Unlock),
    /// Data for tag 48.
    Migrate(Migrate),
    /// Data for tag 49.
    Transfer(Transfer),
    /// Data for tag 50.
    Update(Update),
    /// Data for tag 51.
    Use(Use),
    /// Data for tag 52.
    Verify(Verify),
    /// Data for tag 53.
    Unverify(Unverify),
    /// Data for tag 54.
    Collect(Collect),
    /// Data for tag 55.
    Print(Print),
    /// Data for tag 56.
    Resize(Resize),
    /// Data for tag 57.
    CloseAccounts(CloseAccounts),
}
/// Strict bounded instruction dispatch codec.
#[derive(Clone, Copy, Debug)]
pub struct MplTokenMetadataInstructionDataCodec;
impl Encoder<MplTokenMetadataInstructionData> for MplTokenMetadataInstructionDataCodec {
    fn encoded_len(&self, value: &MplTokenMetadataInstructionData) -> Result<usize, Error> {
        Self::encoded_len_0(value)
    }
    fn write(
        &self,
        value: &MplTokenMetadataInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        Self::write_0(value, writer)
    }
}
impl Decoder<MplTokenMetadataInstructionData> for MplTokenMetadataInstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<MplTokenMetadataInstructionData, Error> {
        let tag = *reader.remaining().first().ok_or(Error::Truncated)?;
        Self::read_0(tag, reader)
    }
}
impl MplTokenMetadataInstructionDataCodec {
    fn encoded_len_0(value: &MplTokenMetadataInstructionData) -> Result<usize, Error> {
        match value {
MplTokenMetadataInstructionData::CreateMetadataAccount(value) => CreateMetadataAccountCodec.encoded_len(value),
MplTokenMetadataInstructionData::UpdateMetadataAccount(value) => UpdateMetadataAccountCodec.encoded_len(value),
MplTokenMetadataInstructionData::DeprecatedCreateMasterEdition(value) => DeprecatedCreateMasterEditionCodec.encoded_len(value),
MplTokenMetadataInstructionData::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(value) => DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec.encoded_len(value),
MplTokenMetadataInstructionData::UpdatePrimarySaleHappenedViaToken(value) => UpdatePrimarySaleHappenedViaTokenCodec.encoded_len(value),
MplTokenMetadataInstructionData::DeprecatedSetReservationList(value) => DeprecatedSetReservationListCodec.encoded_len(value),
MplTokenMetadataInstructionData::DeprecatedCreateReservationList(value) => DeprecatedCreateReservationListCodec.encoded_len(value),
MplTokenMetadataInstructionData::SignMetadata(value) => SignMetadataCodec.encoded_len(value),
MplTokenMetadataInstructionData::DeprecatedMintPrintingTokensViaToken(value) => DeprecatedMintPrintingTokensViaTokenCodec.encoded_len(value),
MplTokenMetadataInstructionData::DeprecatedMintPrintingTokens(value) => DeprecatedMintPrintingTokensCodec.encoded_len(value),
MplTokenMetadataInstructionData::CreateMasterEdition(value) => CreateMasterEditionCodec.encoded_len(value),
MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaToken(value) => MintNewEditionFromMasterEditionViaTokenCodec.encoded_len(value),
MplTokenMetadataInstructionData::ConvertMasterEditionV1ToV2(value) => ConvertMasterEditionV1ToV2Codec.encoded_len(value),
MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaVaultProxy(value) => MintNewEditionFromMasterEditionViaVaultProxyCodec.encoded_len(value),
MplTokenMetadataInstructionData::PuffMetadata(value) => PuffMetadataCodec.encoded_len(value),
MplTokenMetadataInstructionData::UpdateMetadataAccountV2(value) => UpdateMetadataAccountV2Codec.encoded_len(value),
_ => Self::encoded_len_1(value),
}
    }
    fn encoded_len_1(value: &MplTokenMetadataInstructionData) -> Result<usize, Error> {
        match value {
            MplTokenMetadataInstructionData::CreateMetadataAccountV2(value) => {
                CreateMetadataAccountV2Codec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::CreateMasterEditionV3(value) => {
                CreateMasterEditionV3Codec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::VerifyCollection(value) => {
                VerifyCollectionCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::Utilize(value) => UtilizeCodec.encoded_len(value),
            MplTokenMetadataInstructionData::ApproveUseAuthority(value) => {
                ApproveUseAuthorityCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::RevokeUseAuthority(value) => {
                RevokeUseAuthorityCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::UnverifyCollection(value) => {
                UnverifyCollectionCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::ApproveCollectionAuthority(value) => {
                ApproveCollectionAuthorityCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::RevokeCollectionAuthority(value) => {
                RevokeCollectionAuthorityCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::SetAndVerifyCollection(value) => {
                SetAndVerifyCollectionCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::FreezeDelegatedAccount(value) => {
                FreezeDelegatedAccountCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::ThawDelegatedAccount(value) => {
                ThawDelegatedAccountCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::RemoveCreatorVerification(value) => {
                RemoveCreatorVerificationCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::BurnNft(value) => BurnNftCodec.encoded_len(value),
            MplTokenMetadataInstructionData::VerifySizedCollectionItem(value) => {
                VerifySizedCollectionItemCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::UnverifySizedCollectionItem(value) => {
                UnverifySizedCollectionItemCodec.encoded_len(value)
            }
            _ => Self::encoded_len_2(value),
        }
    }
    fn encoded_len_2(value: &MplTokenMetadataInstructionData) -> Result<usize, Error> {
        match value {
            MplTokenMetadataInstructionData::SetAndVerifySizedCollectionItem(value) => {
                SetAndVerifySizedCollectionItemCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::CreateMetadataAccountV3(value) => {
                CreateMetadataAccountV3Codec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::SetCollectionSize(value) => {
                SetCollectionSizeCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::SetTokenStandard(value) => {
                SetTokenStandardCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::BubblegumSetCollectionSize(value) => {
                BubblegumSetCollectionSizeCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::BurnEditionNft(value) => {
                BurnEditionNftCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::CreateEscrowAccount(value) => {
                CreateEscrowAccountCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::CloseEscrowAccount(value) => {
                CloseEscrowAccountCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::TransferOutOfEscrow(value) => {
                TransferOutOfEscrowCodec.encoded_len(value)
            }
            MplTokenMetadataInstructionData::Burn(value) => BurnCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Create(value) => CreateCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Mint(value) => MintCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Delegate(value) => DelegateCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Revoke(value) => RevokeCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Lock(value) => LockCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Unlock(value) => UnlockCodec.encoded_len(value),
            _ => Self::encoded_len_3(value),
        }
    }
    fn encoded_len_3(value: &MplTokenMetadataInstructionData) -> Result<usize, Error> {
        match value {
            MplTokenMetadataInstructionData::Migrate(value) => MigrateCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Transfer(value) => TransferCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Update(value) => UpdateCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Use(value) => UseCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Verify(value) => VerifyCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Unverify(value) => UnverifyCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Collect(value) => CollectCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Print(value) => PrintCodec.encoded_len(value),
            MplTokenMetadataInstructionData::Resize(value) => ResizeCodec.encoded_len(value),
            MplTokenMetadataInstructionData::CloseAccounts(value) => {
                CloseAccountsCodec.encoded_len(value)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn write_0(
        value: &MplTokenMetadataInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
MplTokenMetadataInstructionData::CreateMetadataAccount(value) => CreateMetadataAccountCodec.write(value, writer),
MplTokenMetadataInstructionData::UpdateMetadataAccount(value) => UpdateMetadataAccountCodec.write(value, writer),
MplTokenMetadataInstructionData::DeprecatedCreateMasterEdition(value) => DeprecatedCreateMasterEditionCodec.write(value, writer),
MplTokenMetadataInstructionData::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(value) => DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec.write(value, writer),
MplTokenMetadataInstructionData::UpdatePrimarySaleHappenedViaToken(value) => UpdatePrimarySaleHappenedViaTokenCodec.write(value, writer),
MplTokenMetadataInstructionData::DeprecatedSetReservationList(value) => DeprecatedSetReservationListCodec.write(value, writer),
MplTokenMetadataInstructionData::DeprecatedCreateReservationList(value) => DeprecatedCreateReservationListCodec.write(value, writer),
MplTokenMetadataInstructionData::SignMetadata(value) => SignMetadataCodec.write(value, writer),
MplTokenMetadataInstructionData::DeprecatedMintPrintingTokensViaToken(value) => DeprecatedMintPrintingTokensViaTokenCodec.write(value, writer),
MplTokenMetadataInstructionData::DeprecatedMintPrintingTokens(value) => DeprecatedMintPrintingTokensCodec.write(value, writer),
MplTokenMetadataInstructionData::CreateMasterEdition(value) => CreateMasterEditionCodec.write(value, writer),
MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaToken(value) => MintNewEditionFromMasterEditionViaTokenCodec.write(value, writer),
MplTokenMetadataInstructionData::ConvertMasterEditionV1ToV2(value) => ConvertMasterEditionV1ToV2Codec.write(value, writer),
MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaVaultProxy(value) => MintNewEditionFromMasterEditionViaVaultProxyCodec.write(value, writer),
MplTokenMetadataInstructionData::PuffMetadata(value) => PuffMetadataCodec.write(value, writer),
MplTokenMetadataInstructionData::UpdateMetadataAccountV2(value) => UpdateMetadataAccountV2Codec.write(value, writer),
_ => Self::write_1(value, writer),
}
    }
    fn write_1(
        value: &MplTokenMetadataInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            MplTokenMetadataInstructionData::CreateMetadataAccountV2(value) => {
                CreateMetadataAccountV2Codec.write(value, writer)
            }
            MplTokenMetadataInstructionData::CreateMasterEditionV3(value) => {
                CreateMasterEditionV3Codec.write(value, writer)
            }
            MplTokenMetadataInstructionData::VerifyCollection(value) => {
                VerifyCollectionCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::Utilize(value) => UtilizeCodec.write(value, writer),
            MplTokenMetadataInstructionData::ApproveUseAuthority(value) => {
                ApproveUseAuthorityCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::RevokeUseAuthority(value) => {
                RevokeUseAuthorityCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::UnverifyCollection(value) => {
                UnverifyCollectionCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::ApproveCollectionAuthority(value) => {
                ApproveCollectionAuthorityCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::RevokeCollectionAuthority(value) => {
                RevokeCollectionAuthorityCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::SetAndVerifyCollection(value) => {
                SetAndVerifyCollectionCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::FreezeDelegatedAccount(value) => {
                FreezeDelegatedAccountCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::ThawDelegatedAccount(value) => {
                ThawDelegatedAccountCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::RemoveCreatorVerification(value) => {
                RemoveCreatorVerificationCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::BurnNft(value) => BurnNftCodec.write(value, writer),
            MplTokenMetadataInstructionData::VerifySizedCollectionItem(value) => {
                VerifySizedCollectionItemCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::UnverifySizedCollectionItem(value) => {
                UnverifySizedCollectionItemCodec.write(value, writer)
            }
            _ => Self::write_2(value, writer),
        }
    }
    fn write_2(
        value: &MplTokenMetadataInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            MplTokenMetadataInstructionData::SetAndVerifySizedCollectionItem(value) => {
                SetAndVerifySizedCollectionItemCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::CreateMetadataAccountV3(value) => {
                CreateMetadataAccountV3Codec.write(value, writer)
            }
            MplTokenMetadataInstructionData::SetCollectionSize(value) => {
                SetCollectionSizeCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::SetTokenStandard(value) => {
                SetTokenStandardCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::BubblegumSetCollectionSize(value) => {
                BubblegumSetCollectionSizeCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::BurnEditionNft(value) => {
                BurnEditionNftCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::CreateEscrowAccount(value) => {
                CreateEscrowAccountCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::CloseEscrowAccount(value) => {
                CloseEscrowAccountCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::TransferOutOfEscrow(value) => {
                TransferOutOfEscrowCodec.write(value, writer)
            }
            MplTokenMetadataInstructionData::Burn(value) => BurnCodec.write(value, writer),
            MplTokenMetadataInstructionData::Create(value) => CreateCodec.write(value, writer),
            MplTokenMetadataInstructionData::Mint(value) => MintCodec.write(value, writer),
            MplTokenMetadataInstructionData::Delegate(value) => DelegateCodec.write(value, writer),
            MplTokenMetadataInstructionData::Revoke(value) => RevokeCodec.write(value, writer),
            MplTokenMetadataInstructionData::Lock(value) => LockCodec.write(value, writer),
            MplTokenMetadataInstructionData::Unlock(value) => UnlockCodec.write(value, writer),
            _ => Self::write_3(value, writer),
        }
    }
    fn write_3(
        value: &MplTokenMetadataInstructionData,
        writer: &mut Writer<'_>,
    ) -> Result<(), Error> {
        match value {
            MplTokenMetadataInstructionData::Migrate(value) => MigrateCodec.write(value, writer),
            MplTokenMetadataInstructionData::Transfer(value) => TransferCodec.write(value, writer),
            MplTokenMetadataInstructionData::Update(value) => UpdateCodec.write(value, writer),
            MplTokenMetadataInstructionData::Use(value) => UseCodec.write(value, writer),
            MplTokenMetadataInstructionData::Verify(value) => VerifyCodec.write(value, writer),
            MplTokenMetadataInstructionData::Unverify(value) => UnverifyCodec.write(value, writer),
            MplTokenMetadataInstructionData::Collect(value) => CollectCodec.write(value, writer),
            MplTokenMetadataInstructionData::Print(value) => PrintCodec.write(value, writer),
            MplTokenMetadataInstructionData::Resize(value) => ResizeCodec.write(value, writer),
            MplTokenMetadataInstructionData::CloseAccounts(value) => {
                CloseAccountsCodec.write(value, writer)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn read_0(tag: u8, reader: &mut Reader<'_>) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
INSTRUCTION_TAG_0 => Ok(MplTokenMetadataInstructionData::CreateMetadataAccount(CreateMetadataAccountCodec.read(reader)?)),
INSTRUCTION_TAG_1 => Ok(MplTokenMetadataInstructionData::UpdateMetadataAccount(UpdateMetadataAccountCodec.read(reader)?)),
INSTRUCTION_TAG_2 => Ok(MplTokenMetadataInstructionData::DeprecatedCreateMasterEdition(DeprecatedCreateMasterEditionCodec.read(reader)?)),
INSTRUCTION_TAG_3 => Ok(MplTokenMetadataInstructionData::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenCodec.read(reader)?)),
INSTRUCTION_TAG_4 => Ok(MplTokenMetadataInstructionData::UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaTokenCodec.read(reader)?)),
INSTRUCTION_TAG_5 => Ok(MplTokenMetadataInstructionData::DeprecatedSetReservationList(DeprecatedSetReservationListCodec.read(reader)?)),
INSTRUCTION_TAG_6 => Ok(MplTokenMetadataInstructionData::DeprecatedCreateReservationList(DeprecatedCreateReservationListCodec.read(reader)?)),
INSTRUCTION_TAG_7 => Ok(MplTokenMetadataInstructionData::SignMetadata(SignMetadataCodec.read(reader)?)),
INSTRUCTION_TAG_8 => Ok(MplTokenMetadataInstructionData::DeprecatedMintPrintingTokensViaToken(DeprecatedMintPrintingTokensViaTokenCodec.read(reader)?)),
INSTRUCTION_TAG_9 => Ok(MplTokenMetadataInstructionData::DeprecatedMintPrintingTokens(DeprecatedMintPrintingTokensCodec.read(reader)?)),
INSTRUCTION_TAG_10 => Ok(MplTokenMetadataInstructionData::CreateMasterEdition(CreateMasterEditionCodec.read(reader)?)),
INSTRUCTION_TAG_11 => Ok(MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaTokenCodec.read(reader)?)),
INSTRUCTION_TAG_12 => Ok(MplTokenMetadataInstructionData::ConvertMasterEditionV1ToV2(ConvertMasterEditionV1ToV2Codec.read(reader)?)),
INSTRUCTION_TAG_13 => Ok(MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaVaultProxy(MintNewEditionFromMasterEditionViaVaultProxyCodec.read(reader)?)),
INSTRUCTION_TAG_14 => Ok(MplTokenMetadataInstructionData::PuffMetadata(PuffMetadataCodec.read(reader)?)),
INSTRUCTION_TAG_15 => Ok(MplTokenMetadataInstructionData::UpdateMetadataAccountV2(UpdateMetadataAccountV2Codec.read(reader)?)),
_ => Self::read_1(tag, reader),
}
    }
    fn read_1(tag: u8, reader: &mut Reader<'_>) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_16 => Ok(MplTokenMetadataInstructionData::CreateMetadataAccountV2(
                CreateMetadataAccountV2Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_17 => Ok(MplTokenMetadataInstructionData::CreateMasterEditionV3(
                CreateMasterEditionV3Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_18 => Ok(MplTokenMetadataInstructionData::VerifyCollection(
                VerifyCollectionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_19 => Ok(MplTokenMetadataInstructionData::Utilize(
                UtilizeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_20 => Ok(MplTokenMetadataInstructionData::ApproveUseAuthority(
                ApproveUseAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_21 => Ok(MplTokenMetadataInstructionData::RevokeUseAuthority(
                RevokeUseAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_22 => Ok(MplTokenMetadataInstructionData::UnverifyCollection(
                UnverifyCollectionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_23 => Ok(MplTokenMetadataInstructionData::ApproveCollectionAuthority(
                ApproveCollectionAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_24 => Ok(MplTokenMetadataInstructionData::RevokeCollectionAuthority(
                RevokeCollectionAuthorityCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_25 => Ok(MplTokenMetadataInstructionData::SetAndVerifyCollection(
                SetAndVerifyCollectionCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_26 => Ok(MplTokenMetadataInstructionData::FreezeDelegatedAccount(
                FreezeDelegatedAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_27 => Ok(MplTokenMetadataInstructionData::ThawDelegatedAccount(
                ThawDelegatedAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_28 => Ok(MplTokenMetadataInstructionData::RemoveCreatorVerification(
                RemoveCreatorVerificationCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_29 => Ok(MplTokenMetadataInstructionData::BurnNft(
                BurnNftCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_30 => Ok(MplTokenMetadataInstructionData::VerifySizedCollectionItem(
                VerifySizedCollectionItemCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_31 => Ok(
                MplTokenMetadataInstructionData::UnverifySizedCollectionItem(
                    UnverifySizedCollectionItemCodec.read(reader)?,
                ),
            ),
            _ => Self::read_2(tag, reader),
        }
    }
    fn read_2(tag: u8, reader: &mut Reader<'_>) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_32 => Ok(
                MplTokenMetadataInstructionData::SetAndVerifySizedCollectionItem(
                    SetAndVerifySizedCollectionItemCodec.read(reader)?,
                ),
            ),
            INSTRUCTION_TAG_33 => Ok(MplTokenMetadataInstructionData::CreateMetadataAccountV3(
                CreateMetadataAccountV3Codec.read(reader)?,
            )),
            INSTRUCTION_TAG_34 => Ok(MplTokenMetadataInstructionData::SetCollectionSize(
                SetCollectionSizeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_35 => Ok(MplTokenMetadataInstructionData::SetTokenStandard(
                SetTokenStandardCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_36 => Ok(MplTokenMetadataInstructionData::BubblegumSetCollectionSize(
                BubblegumSetCollectionSizeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_37 => Ok(MplTokenMetadataInstructionData::BurnEditionNft(
                BurnEditionNftCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_38 => Ok(MplTokenMetadataInstructionData::CreateEscrowAccount(
                CreateEscrowAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_39 => Ok(MplTokenMetadataInstructionData::CloseEscrowAccount(
                CloseEscrowAccountCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_40 => Ok(MplTokenMetadataInstructionData::TransferOutOfEscrow(
                TransferOutOfEscrowCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_41 => Ok(MplTokenMetadataInstructionData::Burn(
                BurnCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_42 => Ok(MplTokenMetadataInstructionData::Create(
                CreateCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_43 => Ok(MplTokenMetadataInstructionData::Mint(
                MintCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_44 => Ok(MplTokenMetadataInstructionData::Delegate(
                DelegateCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_45 => Ok(MplTokenMetadataInstructionData::Revoke(
                RevokeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_46 => Ok(MplTokenMetadataInstructionData::Lock(
                LockCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_47 => Ok(MplTokenMetadataInstructionData::Unlock(
                UnlockCodec.read(reader)?,
            )),
            _ => Self::read_3(tag, reader),
        }
    }
    fn read_3(tag: u8, reader: &mut Reader<'_>) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_48 => Ok(MplTokenMetadataInstructionData::Migrate(
                MigrateCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_49 => Ok(MplTokenMetadataInstructionData::Transfer(
                TransferCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_50 => Ok(MplTokenMetadataInstructionData::Update(
                UpdateCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_51 => Ok(MplTokenMetadataInstructionData::Use(UseCodec.read(reader)?)),
            INSTRUCTION_TAG_52 => Ok(MplTokenMetadataInstructionData::Verify(
                VerifyCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_53 => Ok(MplTokenMetadataInstructionData::Unverify(
                UnverifyCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_54 => Ok(MplTokenMetadataInstructionData::Collect(
                CollectCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_55 => Ok(MplTokenMetadataInstructionData::Print(
                PrintCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_56 => Ok(MplTokenMetadataInstructionData::Resize(
                ResizeCodec.read(reader)?,
            )),
            INSTRUCTION_TAG_57 => Ok(MplTokenMetadataInstructionData::CloseAccounts(
                CloseAccountsCodec.read(reader)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
    fn parse_0(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
INSTRUCTION_TAG_0 => Ok(MplTokenMetadataInstructionData::CreateMetadataAccount(CreateMetadataAccount::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_1 => Ok(MplTokenMetadataInstructionData::UpdateMetadataAccount(UpdateMetadataAccount::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_2 => Ok(MplTokenMetadataInstructionData::DeprecatedCreateMasterEdition(DeprecatedCreateMasterEdition::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_3 => Ok(MplTokenMetadataInstructionData::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(DeprecatedMintNewEditionFromMasterEditionViaPrintingToken::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_4 => Ok(MplTokenMetadataInstructionData::UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaToken::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_5 => Ok(MplTokenMetadataInstructionData::DeprecatedSetReservationList(DeprecatedSetReservationList::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_6 => Ok(MplTokenMetadataInstructionData::DeprecatedCreateReservationList(DeprecatedCreateReservationList::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_7 => Ok(MplTokenMetadataInstructionData::SignMetadata(SignMetadata::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_8 => Ok(MplTokenMetadataInstructionData::DeprecatedMintPrintingTokensViaToken(DeprecatedMintPrintingTokensViaToken::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_9 => Ok(MplTokenMetadataInstructionData::DeprecatedMintPrintingTokens(DeprecatedMintPrintingTokens::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_10 => Ok(MplTokenMetadataInstructionData::CreateMasterEdition(CreateMasterEdition::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_11 => Ok(MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaToken::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_12 => Ok(MplTokenMetadataInstructionData::ConvertMasterEditionV1ToV2(ConvertMasterEditionV1ToV2::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_13 => Ok(MplTokenMetadataInstructionData::MintNewEditionFromMasterEditionViaVaultProxy(MintNewEditionFromMasterEditionViaVaultProxy::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_14 => Ok(MplTokenMetadataInstructionData::PuffMetadata(PuffMetadata::parse_with_program(program,instruction,limits)?)),
INSTRUCTION_TAG_15 => Ok(MplTokenMetadataInstructionData::UpdateMetadataAccountV2(UpdateMetadataAccountV2::parse_with_program(program,instruction,limits)?)),
_ => Self::parse_1(tag, program, instruction, limits),
}
    }
    fn parse_1(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_16 => Ok(MplTokenMetadataInstructionData::CreateMetadataAccountV2(
                CreateMetadataAccountV2::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_17 => Ok(MplTokenMetadataInstructionData::CreateMasterEditionV3(
                CreateMasterEditionV3::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_18 => Ok(MplTokenMetadataInstructionData::VerifyCollection(
                VerifyCollection::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_19 => Ok(MplTokenMetadataInstructionData::Utilize(
                Utilize::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_20 => Ok(MplTokenMetadataInstructionData::ApproveUseAuthority(
                ApproveUseAuthority::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_21 => Ok(MplTokenMetadataInstructionData::RevokeUseAuthority(
                RevokeUseAuthority::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_22 => Ok(MplTokenMetadataInstructionData::UnverifyCollection(
                UnverifyCollection::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_23 => Ok(MplTokenMetadataInstructionData::ApproveCollectionAuthority(
                ApproveCollectionAuthority::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_24 => Ok(MplTokenMetadataInstructionData::RevokeCollectionAuthority(
                RevokeCollectionAuthority::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_25 => Ok(MplTokenMetadataInstructionData::SetAndVerifyCollection(
                SetAndVerifyCollection::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_26 => Ok(MplTokenMetadataInstructionData::FreezeDelegatedAccount(
                FreezeDelegatedAccount::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_27 => Ok(MplTokenMetadataInstructionData::ThawDelegatedAccount(
                ThawDelegatedAccount::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_28 => Ok(MplTokenMetadataInstructionData::RemoveCreatorVerification(
                RemoveCreatorVerification::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_29 => Ok(MplTokenMetadataInstructionData::BurnNft(
                BurnNft::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_30 => Ok(MplTokenMetadataInstructionData::VerifySizedCollectionItem(
                VerifySizedCollectionItem::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_31 => Ok(
                MplTokenMetadataInstructionData::UnverifySizedCollectionItem(
                    UnverifySizedCollectionItem::parse_with_program(program, instruction, limits)?,
                ),
            ),
            _ => Self::parse_2(tag, program, instruction, limits),
        }
    }
    fn parse_2(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_32 => Ok(
                MplTokenMetadataInstructionData::SetAndVerifySizedCollectionItem(
                    SetAndVerifySizedCollectionItem::parse_with_program(
                        program,
                        instruction,
                        limits,
                    )?,
                ),
            ),
            INSTRUCTION_TAG_33 => Ok(MplTokenMetadataInstructionData::CreateMetadataAccountV3(
                CreateMetadataAccountV3::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_34 => Ok(MplTokenMetadataInstructionData::SetCollectionSize(
                SetCollectionSize::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_35 => Ok(MplTokenMetadataInstructionData::SetTokenStandard(
                SetTokenStandard::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_36 => Ok(MplTokenMetadataInstructionData::BubblegumSetCollectionSize(
                BubblegumSetCollectionSize::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_37 => Ok(MplTokenMetadataInstructionData::BurnEditionNft(
                BurnEditionNft::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_38 => Ok(MplTokenMetadataInstructionData::CreateEscrowAccount(
                CreateEscrowAccount::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_39 => Ok(MplTokenMetadataInstructionData::CloseEscrowAccount(
                CloseEscrowAccount::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_40 => Ok(MplTokenMetadataInstructionData::TransferOutOfEscrow(
                TransferOutOfEscrow::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_41 => Ok(MplTokenMetadataInstructionData::Burn(
                Burn::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_42 => Ok(MplTokenMetadataInstructionData::Create(
                Create::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_43 => Ok(MplTokenMetadataInstructionData::Mint(
                Mint::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_44 => Ok(MplTokenMetadataInstructionData::Delegate(
                Delegate::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_45 => Ok(MplTokenMetadataInstructionData::Revoke(
                Revoke::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_46 => Ok(MplTokenMetadataInstructionData::Lock(
                Lock::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_47 => Ok(MplTokenMetadataInstructionData::Unlock(
                Unlock::parse_with_program(program, instruction, limits)?,
            )),
            _ => Self::parse_3(tag, program, instruction, limits),
        }
    }
    fn parse_3(
        tag: u8,
        program: Address,
        instruction: &Instruction,
        limits: Limits,
    ) -> Result<MplTokenMetadataInstructionData, Error> {
        match tag {
            INSTRUCTION_TAG_48 => Ok(MplTokenMetadataInstructionData::Migrate(
                Migrate::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_49 => Ok(MplTokenMetadataInstructionData::Transfer(
                Transfer::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_50 => Ok(MplTokenMetadataInstructionData::Update(
                Update::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_51 => Ok(MplTokenMetadataInstructionData::Use(
                Use::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_52 => Ok(MplTokenMetadataInstructionData::Verify(
                Verify::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_53 => Ok(MplTokenMetadataInstructionData::Unverify(
                Unverify::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_54 => Ok(MplTokenMetadataInstructionData::Collect(
                Collect::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_55 => Ok(MplTokenMetadataInstructionData::Print(
                Print::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_56 => Ok(MplTokenMetadataInstructionData::Resize(
                Resize::parse_with_program(program, instruction, limits)?,
            )),
            INSTRUCTION_TAG_57 => Ok(MplTokenMetadataInstructionData::CloseAccounts(
                CloseAccounts::parse_with_program(program, instruction, limits)?,
            )),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}
/// Identifies a tag without asserting the full payload is valid.
pub fn identify_instruction(data: &[u8]) -> Result<u8, Error> {
    let tag = *data.first().ok_or(Error::Truncated)?;
    const TAGS: &[u8] = &[
        INSTRUCTION_TAG_0,
        INSTRUCTION_TAG_1,
        INSTRUCTION_TAG_2,
        INSTRUCTION_TAG_3,
        INSTRUCTION_TAG_4,
        INSTRUCTION_TAG_5,
        INSTRUCTION_TAG_6,
        INSTRUCTION_TAG_7,
        INSTRUCTION_TAG_8,
        INSTRUCTION_TAG_9,
        INSTRUCTION_TAG_10,
        INSTRUCTION_TAG_11,
        INSTRUCTION_TAG_12,
        INSTRUCTION_TAG_13,
        INSTRUCTION_TAG_14,
        INSTRUCTION_TAG_15,
        INSTRUCTION_TAG_16,
        INSTRUCTION_TAG_17,
        INSTRUCTION_TAG_18,
        INSTRUCTION_TAG_19,
        INSTRUCTION_TAG_20,
        INSTRUCTION_TAG_21,
        INSTRUCTION_TAG_22,
        INSTRUCTION_TAG_23,
        INSTRUCTION_TAG_24,
        INSTRUCTION_TAG_25,
        INSTRUCTION_TAG_26,
        INSTRUCTION_TAG_27,
        INSTRUCTION_TAG_28,
        INSTRUCTION_TAG_29,
        INSTRUCTION_TAG_30,
        INSTRUCTION_TAG_31,
        INSTRUCTION_TAG_32,
        INSTRUCTION_TAG_33,
        INSTRUCTION_TAG_34,
        INSTRUCTION_TAG_35,
        INSTRUCTION_TAG_36,
        INSTRUCTION_TAG_37,
        INSTRUCTION_TAG_38,
        INSTRUCTION_TAG_39,
        INSTRUCTION_TAG_40,
        INSTRUCTION_TAG_41,
        INSTRUCTION_TAG_42,
        INSTRUCTION_TAG_43,
        INSTRUCTION_TAG_44,
        INSTRUCTION_TAG_45,
        INSTRUCTION_TAG_46,
        INSTRUCTION_TAG_47,
        INSTRUCTION_TAG_48,
        INSTRUCTION_TAG_49,
        INSTRUCTION_TAG_50,
        INSTRUCTION_TAG_51,
        INSTRUCTION_TAG_52,
        INSTRUCTION_TAG_53,
        INSTRUCTION_TAG_54,
        INSTRUCTION_TAG_55,
        INSTRUCTION_TAG_56,
        INSTRUCTION_TAG_57,
    ];

    if !TAGS.contains(&tag) {
        return Err(Error::InvalidDiscriminator);
    }
    Ok(tag)
}
/// Strictly parses a canonical program invocation.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<MplTokenMetadataInstructionData, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Strictly parses an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<MplTokenMetadataInstructionData, Error> {
    MplTokenMetadataInstructionDataCodec::parse_0(
        identify_instruction(&instruction.data)?,
        program,
        instruction,
        limits,
    )
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
use solana_kit_core::codecs::Tuple;
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
impl core::hash::Hash for Data {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seller_fee_basis_points, state);
        core::hash::Hash::hash(&self.creators, state);
    }
}
impl core::fmt::Display for Data {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DataV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seller_fee_basis_points, state);
        core::hash::Hash::hash(&self.creators, state);
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.uses, state);
    }
}
impl core::fmt::Display for DataV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AssetData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.name, state);
        core::hash::Hash::hash(&self.symbol, state);
        core::hash::Hash::hash(&self.uri, state);
        core::hash::Hash::hash(&self.seller_fee_basis_points, state);
        core::hash::Hash::hash(&self.creators, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.token_standard, state);
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.uses, state);
        core::hash::Hash::hash(&self.collection_details, state);
        core::hash::Hash::hash(&self.rule_set, state);
    }
}
impl core::fmt::Display for AssetData {
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
impl core::hash::Hash for AuthorizationData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.payload, state);
    }
}
impl core::fmt::Display for AuthorizationData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.map, state);
    }
}
impl core::fmt::Display for Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SeedsVec {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.seeds, state);
    }
}
impl core::fmt::Display for SeedsVec {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProofInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.proof, state);
    }
}
impl core::fmt::Display for ProofInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Reservation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.spots_remaining, state);
        core::hash::Hash::hash(&self.total_spots, state);
    }
}
impl core::fmt::Display for Reservation {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ReservationV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.address, state);
        core::hash::Hash::hash(&self.spots_remaining, state);
        core::hash::Hash::hash(&self.total_spots, state);
    }
}
impl core::fmt::Display for ReservationV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetCollectionSizeArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.size, state);
    }
}
impl core::fmt::Display for SetCollectionSizeArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintNewEditionFromMasterEditionViaTokenArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.edition, state);
    }
}
impl core::fmt::Display for MintNewEditionFromMasterEditionViaTokenArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for AuthorityType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Metadata => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Holder => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::MetadataDelegate => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::TokenDelegate => {
                core::hash::Hash::hash(&4usize, state);
            }
        }
    }
}
impl core::fmt::Display for AuthorityType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for HolderDelegateRole {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::PrintDelegate => {
                core::hash::Hash::hash(&0usize, state);
            }
        }
    }
}
impl core::fmt::Display for HolderDelegateRole {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Key {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Uninitialized => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::EditionV1 => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::MasterEditionV1 => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::ReservationListV1 => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::MetadataV1 => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::ReservationListV2 => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::MasterEditionV2 => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::EditionMarker => {
                core::hash::Hash::hash(&7usize, state);
            }
            Self::UseAuthorityRecord => {
                core::hash::Hash::hash(&8usize, state);
            }
            Self::CollectionAuthorityRecord => {
                core::hash::Hash::hash(&9usize, state);
            }
            Self::TokenOwnedEscrow => {
                core::hash::Hash::hash(&10usize, state);
            }
            Self::TokenRecord => {
                core::hash::Hash::hash(&11usize, state);
            }
            Self::MetadataDelegate => {
                core::hash::Hash::hash(&12usize, state);
            }
            Self::EditionMarkerV2 => {
                core::hash::Hash::hash(&13usize, state);
            }
            Self::HolderDelegate => {
                core::hash::Hash::hash(&14usize, state);
            }
        }
    }
}
impl core::fmt::Display for Key {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MetadataDelegateRole {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::AuthorityItem => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Collection => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Use => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Data => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::ProgrammableConfig => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::DataItem => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::CollectionItem => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::ProgrammableConfigItem => {
                core::hash::Hash::hash(&7usize, state);
            }
        }
    }
}
impl core::fmt::Display for MetadataDelegateRole {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MigrationType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CollectionV1 => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::ProgrammableV1 => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for MigrationType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PayloadKey {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Amount => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Authority => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::AuthoritySeeds => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Delegate => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::DelegateSeeds => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::Destination => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::DestinationSeeds => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::Holder => {
                core::hash::Hash::hash(&7usize, state);
            }
            Self::Source => {
                core::hash::Hash::hash(&8usize, state);
            }
            Self::SourceSeeds => {
                core::hash::Hash::hash(&9usize, state);
            }
        }
    }
}
impl core::fmt::Display for PayloadKey {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CollectionV1 => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::SaleV1 => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::TransferV1 => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::DataV1 => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::UtilityV1 => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::StakingV1 => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::StandardV1 => {
                core::hash::Hash::hash(&6usize, state);
            }
            Self::LockedTransferV1 => {
                core::hash::Hash::hash(&7usize, state);
            }
            Self::ProgrammableConfigV1 => {
                core::hash::Hash::hash(&8usize, state);
            }
            Self::MigrationV1 => {
                core::hash::Hash::hash(&9usize, state);
            }
            Self::AuthorityItemV1 => {
                core::hash::Hash::hash(&10usize, state);
            }
            Self::DataItemV1 => {
                core::hash::Hash::hash(&11usize, state);
            }
            Self::CollectionItemV1 => {
                core::hash::Hash::hash(&12usize, state);
            }
            Self::ProgrammableConfigItemV1 => {
                core::hash::Hash::hash(&13usize, state);
            }
            Self::PrintDelegateV1 => {
                core::hash::Hash::hash(&14usize, state);
            }
        }
    }
}
impl core::fmt::Display for RevokeArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenDelegateRole {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Sale => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Transfer => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Utility => {
                core::hash::Hash::hash(&2usize, state);
            }
            Self::Staking => {
                core::hash::Hash::hash(&3usize, state);
            }
            Self::Standard => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::LockedTransfer => {
                core::hash::Hash::hash(&5usize, state);
            }
            Self::Migration => {
                core::hash::Hash::hash(&6usize, state);
            }
        }
    }
}
impl core::fmt::Display for TokenDelegateRole {
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
            Self::ProgrammableNonFungible => {
                core::hash::Hash::hash(&4usize, state);
            }
            Self::ProgrammableNonFungibleEdition => {
                core::hash::Hash::hash(&5usize, state);
            }
        }
    }
}
impl core::fmt::Display for TokenStandard {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenState {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Unlocked => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Locked => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Listed => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for TokenState {
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
impl core::hash::Hash for VerificationArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CreatorV1 => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::CollectionV1 => {
                core::hash::Hash::hash(&1usize, state);
            }
        }
    }
}
impl core::fmt::Display for VerificationArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
    }
}
impl core::fmt::Display for BurnArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for BurnArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionDetailsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.size, state);
    }
}
impl core::fmt::Display for CollectionDetailsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionDetailsV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.padding, state);
    }
}
impl core::fmt::Display for CollectionDetailsV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionDetails {
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
impl core::fmt::Display for CollectionDetails {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionDetailsToggle {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Clear => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Set(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for CollectionDetailsToggle {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionToggle {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Clear => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Set(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for CollectionToggle {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.asset_data, state);
        core::hash::Hash::hash(&self.decimals, state);
        core::hash::Hash::hash(&self.print_supply, state);
    }
}
impl core::fmt::Display for CreateArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for CreateArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for EscrowAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::TokenOwner => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Creator(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for EscrowAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LockArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for LockArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for LockArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for LockArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnlockArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UnlockArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnlockArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for UnlockArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UseArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UseArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UseArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for UseArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for MintArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for MintArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for TransferArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for TransferArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PayloadType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Pubkey(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Seeds(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MerkleProof(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Number(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for PayloadType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PrintArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.edition, state);
    }
}
impl core::fmt::Display for PrintArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PrintArgsV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.edition, state);
    }
}
impl core::fmt::Display for PrintArgsV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PrintArgs {
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
impl core::fmt::Display for PrintArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PrintSupply {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Zero => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Limited(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Unlimited => {
                core::hash::Hash::hash(&2usize, state);
            }
        }
    }
}
impl core::fmt::Display for PrintSupply {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgrammableConfigV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.rule_set, state);
    }
}
impl core::fmt::Display for ProgrammableConfigV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ProgrammableConfig {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for ProgrammableConfig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RuleSetToggle {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Clear => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Set(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for RuleSetToggle {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UsesToggle {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::None => {
                core::hash::Hash::hash(&0usize, state);
            }
            Self::Clear => {
                core::hash::Hash::hash(&1usize, state);
            }
            Self::Set(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for UsesToggle {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsCollectionV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsCollectionV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsSaleV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsSaleV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsTransferV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsTransferV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsDataV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsDataV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsUtilityV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsUtilityV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsStakingV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsStakingV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsStandardV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
    }
}
impl core::fmt::Display for DelegateArgsStandardV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsLockedTransferV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
        core::hash::Hash::hash(&self.locked_address, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsLockedTransferV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsProgrammableConfigV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsProgrammableConfigV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsAuthorityItemV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsAuthorityItemV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsDataItemV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsDataItemV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsCollectionItemV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsCollectionItemV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsProgrammableConfigItemV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsProgrammableConfigItemV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgsPrintDelegateV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for DelegateArgsPrintDelegateV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DelegateArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CollectionV1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SaleV1(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferV1(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DataV1(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UtilityV1(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::StakingV1(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::StandardV1(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::LockedTransferV1(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProgrammableConfigV1(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AuthorityItemV1(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DataItemV1(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CollectionItemV1(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ProgrammableConfigItemV1(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PrintDelegateV1(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for DelegateArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsV1Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_update_authority, state);
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.collection_details, state);
        core::hash::Hash::hash(&self.uses, state);
        core::hash::Hash::hash(&self.rule_set, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsV1Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsUpdateAuthorityV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_update_authority, state);
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.collection_details, state);
        core::hash::Hash::hash(&self.uses, state);
        core::hash::Hash::hash(&self.rule_set, state);
        core::hash::Hash::hash(&self.token_standard, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsUpdateAuthorityV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsAuthorityItemDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.new_update_authority, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.token_standard, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsAuthorityItemDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsCollectionDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsCollectionDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsDataDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsDataDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsProgrammableConfigDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.rule_set, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsProgrammableConfigDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsDataItemDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsDataItemDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsCollectionItemDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsCollectionItemDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgsAsProgrammableConfigItemDelegateV2Payload {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.rule_set, state);
        core::hash::Hash::hash(&self.authorization_data, state);
    }
}
impl core::fmt::Display for UpdateArgsAsProgrammableConfigItemDelegateV2Payload {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateArgs {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::V1(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsUpdateAuthorityV2(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsAuthorityItemDelegateV2(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsCollectionDelegateV2(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsDataDelegateV2(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsProgrammableConfigDelegateV2(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsDataItemDelegateV2(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsCollectionItemDelegateV2(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::AsProgrammableConfigItemDelegateV2(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for UpdateArgs {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Edition {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.parent, state);
        core::hash::Hash::hash(&self.edition, state);
    }
}
impl core::fmt::Display for Edition {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MasterEditionV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.supply, state);
        core::hash::Hash::hash(&self.max_supply, state);
        core::hash::Hash::hash(&self.printing_mint, state);
        core::hash::Hash::hash(&self.one_time_printing_authorization_mint, state);
    }
}
impl core::fmt::Display for MasterEditionV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ReservationListV1 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.master_edition, state);
        core::hash::Hash::hash(&self.supply_snapshot, state);
        core::hash::Hash::hash(&self.reservations, state);
    }
}
impl core::fmt::Display for ReservationListV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Metadata {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.update_authority, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.edition_nonce, state);
        core::hash::Hash::hash(&self.token_standard, state);
        core::hash::Hash::hash(&self.collection, state);
        core::hash::Hash::hash(&self.uses, state);
        core::hash::Hash::hash(&self.collection_details, state);
        core::hash::Hash::hash(&self.programmable_config, state);
    }
}
impl core::fmt::Display for Metadata {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ReservationListV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.master_edition, state);
        core::hash::Hash::hash(&self.supply_snapshot, state);
        core::hash::Hash::hash(&self.reservations, state);
        core::hash::Hash::hash(&self.total_reservation_spots, state);
        core::hash::Hash::hash(&self.current_reservation_spots, state);
    }
}
impl core::fmt::Display for ReservationListV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MasterEditionV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.supply, state);
        core::hash::Hash::hash(&self.max_supply, state);
    }
}
impl core::fmt::Display for MasterEditionV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for EditionMarker {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.ledger, state);
    }
}
impl core::fmt::Display for EditionMarker {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UseAuthorityRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.allowed_uses, state);
        core::hash::Hash::hash(&self.bump, state);
    }
}
impl core::fmt::Display for UseAuthorityRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CollectionAuthorityRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.new_update_authority, state);
    }
}
impl core::fmt::Display for CollectionAuthorityRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenOwnedEscrow {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.base_token, state);
        core::hash::Hash::hash(&self.authority, state);
        core::hash::Hash::hash(&self.bump, state);
    }
}
impl core::fmt::Display for TokenOwnedEscrow {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TokenRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.state, state);
        core::hash::Hash::hash(&self.rule_set_revision, state);
        core::hash::Hash::hash(&self.delegate, state);
        core::hash::Hash::hash(&self.delegate_role, state);
        core::hash::Hash::hash(&self.locked_transfer, state);
    }
}
impl core::fmt::Display for TokenRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MetadataDelegateRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.delegate, state);
        core::hash::Hash::hash(&self.update_authority, state);
    }
}
impl core::fmt::Display for MetadataDelegateRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for EditionMarkerV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.ledger, state);
    }
}
impl core::fmt::Display for EditionMarkerV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for HolderDelegateRecord {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.bump, state);
        core::hash::Hash::hash(&self.mint, state);
        core::hash::Hash::hash(&self.delegate, state);
        core::hash::Hash::hash(&self.update_authority, state);
    }
}
impl core::fmt::Display for HolderDelegateRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateMetadataAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CreateMetadataAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateMetadataAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for UpdateMetadataAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeprecatedCreateMasterEdition {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeprecatedCreateMasterEdition {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdatePrimarySaleHappenedViaToken {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for UpdatePrimarySaleHappenedViaToken {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeprecatedSetReservationList {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeprecatedSetReservationList {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeprecatedCreateReservationList {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeprecatedCreateReservationList {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SignMetadata {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for SignMetadata {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeprecatedMintPrintingTokensViaToken {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeprecatedMintPrintingTokensViaToken {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for DeprecatedMintPrintingTokens {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for DeprecatedMintPrintingTokens {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateMasterEdition {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CreateMasterEdition {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintNewEditionFromMasterEditionViaToken {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(
            &self.mint_new_edition_from_master_edition_via_token_args,
            state,
        );
    }
}
impl core::fmt::Display for MintNewEditionFromMasterEditionViaToken {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ConvertMasterEditionV1ToV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ConvertMasterEditionV1ToV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MintNewEditionFromMasterEditionViaVaultProxy {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(
            &self.mint_new_edition_from_master_edition_via_token_args,
            state,
        );
    }
}
impl core::fmt::Display for MintNewEditionFromMasterEditionViaVaultProxy {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for PuffMetadata {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for PuffMetadata {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UpdateMetadataAccountV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.new_update_authority, state);
        core::hash::Hash::hash(&self.primary_sale_happened, state);
        core::hash::Hash::hash(&self.is_mutable, state);
    }
}
impl core::fmt::Display for UpdateMetadataAccountV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateMetadataAccountV2 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CreateMetadataAccountV2 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateMasterEditionV3 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.max_supply, state);
    }
}
impl core::fmt::Display for CreateMasterEditionV3 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifyCollection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for VerifyCollection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Utilize {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.number_of_uses, state);
    }
}
impl core::fmt::Display for Utilize {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ApproveUseAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.number_of_uses, state);
    }
}
impl core::fmt::Display for ApproveUseAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeUseAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RevokeUseAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnverifyCollection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for UnverifyCollection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ApproveCollectionAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ApproveCollectionAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RevokeCollectionAuthority {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RevokeCollectionAuthority {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetAndVerifyCollection {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for SetAndVerifyCollection {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for FreezeDelegatedAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for FreezeDelegatedAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for ThawDelegatedAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for ThawDelegatedAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for RemoveCreatorVerification {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for RemoveCreatorVerification {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnNft {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for BurnNft {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for VerifySizedCollectionItem {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for VerifySizedCollectionItem {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for UnverifySizedCollectionItem {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for UnverifySizedCollectionItem {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetAndVerifySizedCollectionItem {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for SetAndVerifySizedCollectionItem {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateMetadataAccountV3 {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.data, state);
        core::hash::Hash::hash(&self.is_mutable, state);
        core::hash::Hash::hash(&self.collection_details, state);
    }
}
impl core::fmt::Display for CreateMetadataAccountV3 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetCollectionSize {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.set_collection_size_args, state);
    }
}
impl core::fmt::Display for SetCollectionSize {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for SetTokenStandard {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for SetTokenStandard {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BubblegumSetCollectionSize {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.set_collection_size_args, state);
    }
}
impl core::fmt::Display for BubblegumSetCollectionSize {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for BurnEditionNft {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for BurnEditionNft {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CreateEscrowAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CreateEscrowAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseEscrowAccount {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseEscrowAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for TransferOutOfEscrow {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.amount, state);
    }
}
impl core::fmt::Display for TransferOutOfEscrow {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Burn {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.burn_args, state);
    }
}
impl core::fmt::Display for Burn {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Create {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.create_args, state);
    }
}
impl core::fmt::Display for Create {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Mint {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.mint_args, state);
    }
}
impl core::fmt::Display for Mint {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Delegate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.delegate_args, state);
    }
}
impl core::fmt::Display for Delegate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Revoke {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.revoke_args, state);
    }
}
impl core::fmt::Display for Revoke {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Lock {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.lock_args, state);
    }
}
impl core::fmt::Display for Lock {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Unlock {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.unlock_args, state);
    }
}
impl core::fmt::Display for Unlock {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Migrate {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for Migrate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Transfer {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.transfer_args, state);
    }
}
impl core::fmt::Display for Transfer {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Update {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.update_args, state);
    }
}
impl core::fmt::Display for Update {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Use {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.use_args, state);
    }
}
impl core::fmt::Display for Use {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Verify {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.verification_args, state);
    }
}
impl core::fmt::Display for Verify {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Unverify {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.verification_args, state);
    }
}
impl core::fmt::Display for Unverify {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Collect {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for Collect {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Print {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.print_args, state);
    }
}
impl core::fmt::Display for Print {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for Resize {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for Resize {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for CloseAccounts {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
    }
}
impl core::fmt::Display for CloseAccounts {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MplTokenMetadataInstructionData {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::CreateMetadataAccount(value) => {
                core::hash::Hash::hash(&0usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateMetadataAccount(value) => {
                core::hash::Hash::hash(&1usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeprecatedCreateMasterEdition(value) => {
                core::hash::Hash::hash(&2usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(value) => {
                core::hash::Hash::hash(&3usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdatePrimarySaleHappenedViaToken(value) => {
                core::hash::Hash::hash(&4usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeprecatedSetReservationList(value) => {
                core::hash::Hash::hash(&5usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeprecatedCreateReservationList(value) => {
                core::hash::Hash::hash(&6usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SignMetadata(value) => {
                core::hash::Hash::hash(&7usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeprecatedMintPrintingTokensViaToken(value) => {
                core::hash::Hash::hash(&8usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::DeprecatedMintPrintingTokens(value) => {
                core::hash::Hash::hash(&9usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateMasterEdition(value) => {
                core::hash::Hash::hash(&10usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MintNewEditionFromMasterEditionViaToken(value) => {
                core::hash::Hash::hash(&11usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ConvertMasterEditionV1ToV2(value) => {
                core::hash::Hash::hash(&12usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::MintNewEditionFromMasterEditionViaVaultProxy(value) => {
                core::hash::Hash::hash(&13usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::PuffMetadata(value) => {
                core::hash::Hash::hash(&14usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UpdateMetadataAccountV2(value) => {
                core::hash::Hash::hash(&15usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateMetadataAccountV2(value) => {
                core::hash::Hash::hash(&16usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateMasterEditionV3(value) => {
                core::hash::Hash::hash(&17usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifyCollection(value) => {
                core::hash::Hash::hash(&18usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Utilize(value) => {
                core::hash::Hash::hash(&19usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ApproveUseAuthority(value) => {
                core::hash::Hash::hash(&20usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeUseAuthority(value) => {
                core::hash::Hash::hash(&21usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UnverifyCollection(value) => {
                core::hash::Hash::hash(&22usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ApproveCollectionAuthority(value) => {
                core::hash::Hash::hash(&23usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RevokeCollectionAuthority(value) => {
                core::hash::Hash::hash(&24usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetAndVerifyCollection(value) => {
                core::hash::Hash::hash(&25usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::FreezeDelegatedAccount(value) => {
                core::hash::Hash::hash(&26usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::ThawDelegatedAccount(value) => {
                core::hash::Hash::hash(&27usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::RemoveCreatorVerification(value) => {
                core::hash::Hash::hash(&28usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BurnNft(value) => {
                core::hash::Hash::hash(&29usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::VerifySizedCollectionItem(value) => {
                core::hash::Hash::hash(&30usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::UnverifySizedCollectionItem(value) => {
                core::hash::Hash::hash(&31usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetAndVerifySizedCollectionItem(value) => {
                core::hash::Hash::hash(&32usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateMetadataAccountV3(value) => {
                core::hash::Hash::hash(&33usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetCollectionSize(value) => {
                core::hash::Hash::hash(&34usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::SetTokenStandard(value) => {
                core::hash::Hash::hash(&35usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BubblegumSetCollectionSize(value) => {
                core::hash::Hash::hash(&36usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::BurnEditionNft(value) => {
                core::hash::Hash::hash(&37usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CreateEscrowAccount(value) => {
                core::hash::Hash::hash(&38usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseEscrowAccount(value) => {
                core::hash::Hash::hash(&39usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::TransferOutOfEscrow(value) => {
                core::hash::Hash::hash(&40usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Burn(value) => {
                core::hash::Hash::hash(&41usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Create(value) => {
                core::hash::Hash::hash(&42usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Mint(value) => {
                core::hash::Hash::hash(&43usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Delegate(value) => {
                core::hash::Hash::hash(&44usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Revoke(value) => {
                core::hash::Hash::hash(&45usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Lock(value) => {
                core::hash::Hash::hash(&46usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Unlock(value) => {
                core::hash::Hash::hash(&47usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Migrate(value) => {
                core::hash::Hash::hash(&48usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Transfer(value) => {
                core::hash::Hash::hash(&49usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Update(value) => {
                core::hash::Hash::hash(&50usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Use(value) => {
                core::hash::Hash::hash(&51usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Verify(value) => {
                core::hash::Hash::hash(&52usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Unverify(value) => {
                core::hash::Hash::hash(&53usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Collect(value) => {
                core::hash::Hash::hash(&54usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Print(value) => {
                core::hash::Hash::hash(&55usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::Resize(value) => {
                core::hash::Hash::hash(&56usize, state);
                core::hash::Hash::hash(value, state);
            }
            Self::CloseAccounts(value) => {
                core::hash::Hash::hash(&57usize, state);
                core::hash::Hash::hash(value, state);
            }
        }
    }
}
impl core::fmt::Display for MplTokenMetadataInstructionData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}
impl core::hash::Hash for MplTokenMetadataError {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.code(), state)
    }
}
impl core::fmt::Display for MplTokenMetadataError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message() {
            Some(message) => formatter.write_str(message),
            None => write!(formatter, "unknown program error {}", self.code()),
        }
    }
}
impl core::error::Error for MplTokenMetadataError {}
