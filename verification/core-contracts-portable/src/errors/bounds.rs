use super::{
    ACTIVE, ALLOCATIONS, BTreeMap, Context, ContextError, ContextValue, LARGE_OBJECT, Ordering,
    SMALL_OBJECT,
};

const TOKEN_BALANCE: u128 = 42;
const PREFIX_ENTRIES: usize = 253;
const PREFIX_ALLOCATION_LIMIT: usize = 512;

fn object(count: usize) -> ContextValue {
    ContextValue::Object(
        (0..count)
            .map(|index| (format!("k{index:06}"), ContextValue::Null))
            .collect(),
    )
}

fn rejection_allocations(mut context: Context, value: ContextValue) -> usize {
    let prior = context.clone();
    ALLOCATIONS.store(0, Ordering::Relaxed);
    ACTIVE.store(true, Ordering::Relaxed);
    let result = context.insert("incoming", value);
    ACTIVE.store(false, Ordering::Relaxed);
    let allocations = ALLOCATIONS.load(Ordering::Relaxed);

    assert_eq!(result, Err(ContextError::Limit));
    assert_eq!(context, prior);
    allocations
}

pub fn oversized_objects_in_lists_have_a_fixed_allocation_bound() {
    for depth in [0, 1, 7, 8] {
        let small = rejection_allocations(
            Context::default(),
            super::nested(object(SMALL_OBJECT), depth),
        );
        let large = rejection_allocations(
            Context::default(),
            super::nested(object(LARGE_OBJECT), depth),
        );
        assert_eq!(small, large, "depth {depth}");
        assert!(large <= 4, "depth {depth} allocated {large}");
        println!("depth {depth}: 257 = {small}; 4096 = {large}");
    }
}

pub fn near_exhausted_prior_facts_preserve_atomicity_and_replacement() {
    let mut context = Context::default();
    context
        .insert("prior", ContextValue::List(vec![ContextValue::Null; 254]))
        .unwrap();
    context.insert("incoming", ContextValue::Null).unwrap();
    context.insert("incoming", object(0)).unwrap();
    let prior = context.clone();
    assert_eq!(
        context.insert("incoming", object(1)),
        Err(ContextError::Limit)
    );
    assert_eq!(context, prior);

    let small = rejection_allocations(context.clone(), object(SMALL_OBJECT));
    let large = rejection_allocations(context, object(LARGE_OBJECT));
    assert_eq!(small, large);
    assert!(large <= 4);
}

pub fn small_maps_keep_exact_keys_values_and_early_privacy_errors() {
    let expected = BTreeMap::from([
        (
            "PUBLIC_tokenBalance".into(),
            ContextValue::Unsigned(TOKEN_BALANCE),
        ),
        ("seedCount".into(), ContextValue::Unsigned(3)),
        ("text".into(), ContextValue::Text("é".into())),
    ]);
    let mut context = Context::default();
    context
        .insert("map", ContextValue::Object(expected.clone()))
        .unwrap();
    assert_eq!(context.get("map"), Some(&ContextValue::Object(expected)));
    let prior = context.clone();
    let value = ContextValue::Object(BTreeMap::from([
        ("Api__Key".into(), ContextValue::Null),
        ("later".into(), object(LARGE_OBJECT)),
    ]));
    assert_eq!(context.insert("map", value), Err(ContextError::Key));
    assert_eq!(context, prior);
}

pub fn nested_object_node_budget_is_exact_at_each_valid_depth() {
    for depth in 0..8 {
        let available = 255 - depth;
        Context::default()
            .insert("v", super::nested(object(available), depth))
            .unwrap();
        assert_eq!(
            Context::default().insert("v", super::nested(object(available + 1), depth)),
            Err(ContextError::Limit)
        );
    }
}

pub fn later_objects_use_the_remaining_list_node_budget() {
    let counts = [SMALL_OBJECT, LARGE_OBJECT].map(|count| {
        let value = ContextValue::List(vec![object(PREFIX_ENTRIES), object(count)]);
        rejection_allocations(Context::default(), value)
    });
    assert_eq!(counts[0], counts[1]);
    assert!(counts[1] <= PREFIX_ALLOCATION_LIMIT);
}
