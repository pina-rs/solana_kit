use std::collections::BTreeMap;

use solana_kit_core::errors::Context;
use solana_kit_core::errors::ContextError;
use solana_kit_core::errors::ContextValue;

mod allocation;
pub mod bounds;

use allocation::count_allocations;
pub mod more;

const MAX_NESTED_OBJECT_DEPTH: usize = 7;
const MAX_KEY_BYTES: usize = 64;
const OVER_TEXT_LIMIT: usize = 1025;
const FINAL_STORAGE_TEXT: usize = 975;
const OVER_STORAGE_TEXT: usize = 976;
const CODEC_BUDGET: usize = 8;

const GENERIC_CODE: i64 = 4615001;
const EXISTING_VALUE: u128 = 7;
const SMALL_OBJECT: usize = 257;
const LARGE_OBJECT: usize = 4096;

fn nested(mut value: ContextValue, depth: usize) -> ContextValue {
    for _ in 0..depth {
        value = ContextValue::List(vec![value]);
    }

    value
}

pub fn nested_key_boundaries() {
    for depth in 0..=MAX_NESTED_OBJECT_DEPTH {
        assert_nested_key(depth, MAX_KEY_BYTES, Ok(()));
        assert_nested_key(depth, MAX_KEY_BYTES + 1, Err(ContextError::Limit));
    }
}

fn assert_nested_key(depth: usize, length: usize, expected: Result<(), ContextError>) {
    let value = ContextValue::Object(BTreeMap::from([("k".repeat(length), ContextValue::Null)]));
    let result = Context::default().insert("value", nested(value, depth));
    assert_eq!(result, expected);
}

pub fn exact_context_boundaries_and_atomic_rejection() {
    for (depth, expected) in [(8, Ok(())), (9, Err(ContextError::Limit))] {
        assert_eq!(
            Context::default().insert("v", nested(ContextValue::Null, depth)),
            expected
        );
    }

    for (children, expected) in [(255, Ok(())), (256, Err(ContextError::Limit))] {
        assert_eq!(
            Context::default().insert("v", ContextValue::List(vec![ContextValue::Null; children])),
            expected
        );
    }

    let mut context = Context::default();

    for index in 0..64 {
        context
            .insert(&format!("k{index}"), ContextValue::Null)
            .unwrap();
    }

    let prior = context.clone();
    assert_eq!(
        context.insert("extra", ContextValue::Null),
        Err(ContextError::Limit)
    );
    assert_eq!(context, prior);

    context
        .insert("k0", ContextValue::Text("x".repeat(1024)))
        .unwrap();
    let prior = context.clone();
    assert_eq!(
        context.insert("k0", ContextValue::Text("x".repeat(OVER_TEXT_LIMIT))),
        Err(ContextError::Limit)
    );
    assert_eq!(context, prior);
}

pub fn exact_aggregate_storage_boundary() {
    let mut context = Context::default();

    for index in 0..15 {
        context
            .insert(
                &format!("k{index:02}"),
                ContextValue::Text("x".repeat(1024)),
            )
            .unwrap();
    }

    context
        .insert("last", ContextValue::Text("x".repeat(FINAL_STORAGE_TEXT)))
        .unwrap();
    let prior = context.clone();
    assert_eq!(
        context.insert("last", ContextValue::Text("x".repeat(OVER_STORAGE_TEXT))),
        Err(ContextError::Limit)
    );
    assert_eq!(context, prior);
}

pub fn nested_sensitive_families_are_rejected_and_public_tokens_survive() {
    for key in [
        "PrIvAtE_Key",
        "s_e_c_r_e_t",
        "AUTHORIZATION",
        "Api__Key",
        "ACCESS_TOKEN",
        "refresh_token",
        "id_token",
        "seed_words",
        "password",
        "credential",
        "access-token",
    ] {
        let value = ContextValue::Object(BTreeMap::from([(
            key.into(),
            ContextValue::Text("sensitive".into()),
        )]));
        assert_eq!(
            Context::default().insert("metadata", nested(value, 3)),
            Err(ContextError::Key),
            "{key}"
        );
    }
    for key in [
        "preTokenBalances",
        "post_token_balances",
        "tokenAmount",
        "seedCount",
    ] {
        Context::default()
            .insert(key, ContextValue::Unsigned(1))
            .unwrap();
    }
}

pub fn malformed_codec_failures_are_distinct_public_values() {
    use solana_kit_core::codecs::Bool;
    use solana_kit_core::codecs::Constant;
    use solana_kit_core::codecs::Decoder;
    use solana_kit_core::codecs::Limits;
    use solana_kit_core::codecs::Reader;
    use solana_kit_core::errors::ErrorFacts;
    let bytes = [3, 9];
    let mut reader = Reader::new(&bytes, Limits::new(CODEC_BUDGET, CODEC_BUDGET)).unwrap();
    assert_eq!(
        Bool.read(&mut reader)
            .unwrap_err()
            .error_code()
            .unwrap()
            .value(),
        8078999
    );
    assert_eq!(reader.position(), 1);
    assert_eq!(
        Constant(&[1])
            .read(&mut reader)
            .unwrap_err()
            .error_code()
            .unwrap()
            .value(),
        8078018
    );
    assert_eq!(reader.position(), 2);
    assert_eq!(bytes, [3, 9]);
}

pub fn rpc_i64_extremes_and_local_fnonce_projection() {
    use std::cell::Cell;
    use std::rc::Rc;

    use solana_kit_client::diagnostics::RpcEnumErrorConfig;
    use solana_kit_client::diagnostics::json_rpc_error;
    use solana_kit_client::diagnostics::rpc_enum_error;
    use solana_kit_core::errors::ErrorCode;

    for value in [i64::MIN, i64::MAX] {
        let json = format!("{{\"code\":{value},\"message\":\"failed\"}}");
        assert_eq!(json_rpc_error(&json).unwrap().code().value(), value);
    }

    for value in ["9223372036854775808", "-9223372036854775809", "1.25"] {
        assert!(json_rpc_error(&format!("{{\"code\":{value},\"message\":\"failed\"}}")).is_err());
    }

    let calls = Rc::new(Cell::new(0));
    let observed = calls.clone();
    let consumed = String::from("owned");
    let error = rpc_enum_error(
        RpcEnumErrorConfig {
            base: ErrorCode::from_value(GENERIC_CODE),
            names: &["Second", "First"],
        },
        "\"First\"",
        move |code, name, payload| {
            drop(consumed);
            observed.set(observed.get() + 1);
            assert_eq!(code.value(), 4615002);
            assert_eq!(name, "First");
            assert_eq!(payload, None);
            Ok(Context::default())
        },
    )
    .unwrap();
    assert_eq!(error.code().value(), 4615002);
    assert_eq!(calls.get(), 1);
}

fn rejected_object_allocations(count: usize) -> usize {
    let entries = (0..count)
        .map(|index| (format!("k{index:06}"), ContextValue::Null))
        .collect();
    let mut context = Context::default();
    context
        .insert("existing", ContextValue::Unsigned(EXISTING_VALUE))
        .unwrap();
    let prior = context.clone();

    let (result, allocations) =
        count_allocations(|| context.insert("incoming", ContextValue::Object(entries)));

    assert_eq!(result, Err(ContextError::Limit));
    assert_eq!(context, prior);
    allocations
}

pub fn rejected_nested_object_allocation_is_bounded_by_node_budget() {
    let small = rejected_object_allocations(SMALL_OBJECT);
    let large = rejected_object_allocations(LARGE_OBJECT);
    println!("insert-only allocations for 257 keys = {small}; for 4096 keys = {large}");
    assert!(
        large <= 1024,
        "rejected input must not allocate per attacker-controlled object entry; observed {large}"
    );
}
