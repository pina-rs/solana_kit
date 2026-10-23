//! Ordinary downstream behavior checks, runnable outside repository configuration.
use solana_kit_client::{
    diagnostics::{json_rpc_error, transaction_error},
    methods::submission::TransactionFailure,
};
use solana_kit_core::{
    Address,
    accounts::MaybeAccount,
    codecs::{Bool, Constant, Decoder, Error, Limits, Number, Reader, ensure_number_range},
    crypto::find_program_address,
    errors::{ContextValue, Diagnostic, ErrorCode, ErrorFacts},
};
use std::error::Error as _;
const BUDGET: usize = 64;
const OUT_OF_RANGE: u8 = 9;
const MAXIMUM: u8 = 8;

fn main() {
    decode_failures();
    execution_failures();
    let missing = MaybeAccount::<()>::Missing(Address::from_bytes([0; 32]))
        .require()
        .unwrap_err();
    assert_eq!(
        missing.error_code(),
        Some(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND)
    );
    let failure = find_program_address(&[&[42; 33]], &Address::from_bytes([0; 32])).unwrap_err();
    assert_eq!(
        failure.error_code(),
        Some(ErrorCode::ADDRESSES_MAX_PDA_SEED_LENGTH_EXCEEDED)
    );
    assert_eq!(
        Address::try_from(&[1, 2][..]).unwrap_err().error_code(),
        Some(ErrorCode::ADDRESSES_INVALID_BYTE_LENGTH)
    );
    println!("ordinary consumer: malformed codec/PDA/account/RPC/execution contracts passed");
}

fn decode_failures() {
    let limits = Limits::new(BUDGET, BUDGET);
    let bytes = [255, 8, 9];
    let mut reader = Reader::new(&bytes, limits).unwrap();
    let boolean = Bool.read(&mut reader).unwrap_err();
    assert_eq!(boolean, Error::InvalidBoolean { value: 255 });
    assert_eq!(
        boolean.error_context().get("value"),
        Some(&ContextValue::Unsigned(255))
    );
    assert_eq!(reader.position(), 1);
    assert_eq!(
        Constant(&[1, 2]).read(&mut reader).unwrap_err(),
        Error::InvalidConstant {
            offset: 1,
            length: 2
        }
    );

    assert_eq!(reader.position(), 3);
    assert_eq!(bytes, [255, 8, 9]);
    assert_eq!(
        Number::<u32>::LE
            .decode(&bytes, limits)
            .unwrap_err()
            .error_code(),
        Some(ErrorCode::CODECS_INVALID_BYTE_LENGTH)
    );
    assert_eq!(
        ensure_number_range(&OUT_OF_RANGE, &0, &MAXIMUM)
            .unwrap_err()
            .error_code(),
        Some(ErrorCode::CODECS_NUMBER_OUT_OF_RANGE)
    );
}

fn execution_failures() {
    let rpc = json_rpc_error(r#"{"code":9223372036854775807,"message":"opaque"}"#).unwrap();
    assert_eq!(rpc.code().value(), i64::MAX);
    let preflight = json_rpc_error(r#"{"code":-32002,"message":"failed","data":{"err":{"InstructionError":[7,{"Custom":4294967295}]}}}"#).unwrap();
    let cause = preflight
        .simulation_cause()
        .downcast_ref::<Diagnostic>()
        .unwrap();
    assert_eq!(cause.code(), ErrorCode::INSTRUCTION_ERROR_CUSTOM);
    assert_eq!(
        cause.context().get("code"),
        Some(&ContextValue::Unsigned(u32::MAX.into()))
    );
    assert!(
        cause
            .source()
            .unwrap()
            .downcast_ref::<TransactionFailure>()
            .is_some()
    );

    let future =
        transaction_error(r#"{"InstructionError":[255,{"Future":9223372036854775807}]}"#).unwrap();
    assert_eq!(future.code(), ErrorCode::INSTRUCTION_ERROR_UNKNOWN);
    assert_eq!(
        future.context().get("instructionErrorContext"),
        Some(&ContextValue::Integer(i64::MAX.into()))
    );
    assert!(transaction_error(r#"{"InstructionError":[0,{"Custom":4294967296}]}"#).is_err());
}
