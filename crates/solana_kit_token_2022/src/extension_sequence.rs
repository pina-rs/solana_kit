// Public adapter over the unchanged shared generated TLV collection codec.

/// Canonical TLV remainder sequence, including termination and shared decode budgets.
/// Zero-tag values are valid standalone extensions but cannot be sequence entries.
#[derive(Clone, Copy, Debug)]
pub struct ExtensionsCodec;

fn extension_sequence() -> TlvSequence<ExtensionCodec, fn(&Extension) -> bool> {
    TlvSequence(ExtensionCodec, |value| {
        matches!(value, Extension::Uninitialized)
    })
}

impl Encoder<Vec<Extension>> for ExtensionsCodec {
    fn encoded_len(&self, value: &Vec<Extension>) -> Result<usize, Error> {
        extension_sequence().encoded_len(value)
    }
    fn write(&self, value: &Vec<Extension>, writer: &mut Writer<'_>) -> Result<(), Error> {
        extension_sequence().write(value, writer)
    }
}

impl Decoder<Vec<Extension>> for ExtensionsCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<Extension>, Error> {
        extension_sequence().read(reader)
    }
}
pub(crate) fn extension_identity(extension: &Extension) -> Result<(u16, bool), Error> {
    use Extension::*;
    Ok(match extension {
        Uninitialized => return Err(Error::InvalidValue),
        TransferFeeConfig(_) => (EXTENSION_TAG_1, true),
        TransferFeeAmount(_) => (EXTENSION_TAG_2, false),
        MintCloseAuthority(_) => (EXTENSION_TAG_3, true),
        ConfidentialTransferMint(_) => (EXTENSION_TAG_4, true),
        ConfidentialTransferAccount(_) => (EXTENSION_TAG_5, false),
        DefaultAccountState(_) => (EXTENSION_TAG_6, true),
        ImmutableOwner(_) => (EXTENSION_TAG_7, false),
        MemoTransfer(_) => (EXTENSION_TAG_8, false),
        NonTransferable(_) => (EXTENSION_TAG_9, true),
        InterestBearingConfig(_) => (EXTENSION_TAG_10, true),
        CpiGuard(_) => (EXTENSION_TAG_11, false),
        PermanentDelegate(_) => (EXTENSION_TAG_12, true),
        NonTransferableAccount(_) => (EXTENSION_TAG_13, false),
        TransferHook(_) => (EXTENSION_TAG_14, true),
        TransferHookAccount(_) => (EXTENSION_TAG_15, false),
        ConfidentialTransferFee(_) => (EXTENSION_TAG_16, true),
        ConfidentialTransferFeeAmount(_) => (EXTENSION_TAG_17, false),
        MetadataPointer(_) => (EXTENSION_TAG_18, true),
        TokenMetadata(_) => (EXTENSION_TAG_19, true),
        GroupPointer(_) => (EXTENSION_TAG_20, true),
        TokenGroup(_) => (EXTENSION_TAG_21, true),
        GroupMemberPointer(_) => (EXTENSION_TAG_22, true),
        TokenGroupMember(_) => (EXTENSION_TAG_23, true),
        ConfidentialMintBurn(_) => (EXTENSION_TAG_24, true),
        ScaledUiAmountConfig(_) => (EXTENSION_TAG_25, true),
        PausableConfig(_) => (EXTENSION_TAG_26, true),
        PausableAccount(_) => (EXTENSION_TAG_27, false),
        PermissionedBurn(_) => (EXTENSION_TAG_28, true),
    })
}
