use solana_kit_core::errors::{Context, ContextError, ContextValue};

#[test]
fn nine_nested_lists_exceed_the_public_depth_limit() {
    let mut value = ContextValue::Null;

    for _ in 0..9 {
        value = ContextValue::List(vec![value]);
    }

    assert_eq!(
        Context::default().insert("value", value),
        Err(ContextError::Limit)
    );
}
