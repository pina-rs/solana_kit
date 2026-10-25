use alloc::{vec, vec::Vec};
use solana_kit_core::{
    codecs::{
        Count, Counted, Decoder, Encoder, Error, ItemSentinel, Limits, Mapped, Number,
        OptionalCount, OrderedMapCodec, OrderedSetCodec, Sequence, Termination, Tuple,
    },
    collections::{OrderedMap, OrderedSet},
};

const LIMITS: Limits = Limits::new(64, 64);

fn map_roundtrip<C>(sequence: C, bytes: &[u8])
where
    C: Encoder<[(u8, u8)]> + Decoder<Vec<(u8, u8)>>,
{
    let value = OrderedMap::from_entries(vec![(2, 20), (1, 10)]);
    let codec = OrderedMapCodec(sequence);
    assert_eq!(codec.encode(&value, 64).unwrap(), bytes);
    assert_eq!(
        codec.decode(bytes, LIMITS).unwrap().as_slice(),
        value.as_slice()
    );
}

fn set_roundtrip<C>(sequence: C, bytes: &[u8])
where
    C: Encoder<[u8]> + Decoder<Vec<u8>>,
{
    let value = OrderedSet::from_values(vec![2, 1]);
    let codec = OrderedSetCodec(sequence);
    assert_eq!(codec.encode(&value, 64).unwrap(), bytes);
    assert_eq!(
        codec.decode(bytes, LIMITS).unwrap().as_slice(),
        value.as_slice()
    );
}

/// Each framing strategy delegates to the same existing sequence contract.
pub fn count_strategies() {
    let pairs = Tuple(Number::<u8>::LE, Number::<u8>::LE);

    map_roundtrip(
        Sequence {
            item: pairs,
            count: Count::U32,
        },
        &[2, 0, 0, 0, 2, 20, 1, 10],
    );
    set_roundtrip(
        Sequence {
            item: Number::<u8>::LE,
            count: Count::U32,
        },
        &[2, 0, 0, 0, 2, 1],
    );
    map_roundtrip(
        Sequence {
            item: pairs,
            count: Count::Fixed(2),
        },
        &[2, 20, 1, 10],
    );
    set_roundtrip(
        Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(2),
        },
        &[2, 1],
    );
    map_roundtrip(
        Sequence {
            item: pairs,
            count: Count::Remainder,
        },
        &[2, 20, 1, 10],
    );
    set_roundtrip(
        Sequence {
            item: Number::<u8>::LE,
            count: Count::Remainder,
        },
        &[2, 1],
    );

    selected_prefixes();
    empty_default();
}

fn selected_prefixes() {
    let narrow = Mapped::new(
        Number::<u8>::LE,
        |count: &u32| u8::try_from(*count).map_err(|_| Error::Overflow),
        |count| Ok(u32::from(count)),
    );
    let wide = Mapped::new(
        Number::<u64>::LE,
        |count: &u32| Ok(u64::from(*count)),
        |count| u32::try_from(count).map_err(|_| Error::Overflow),
    );
    let pairs = Tuple(Number::<u8>::LE, Number::<u8>::LE);
    map_roundtrip(
        Counted {
            prefix: narrow,
            item: pairs,
        },
        &[2, 2, 20, 1, 10],
    );
    set_roundtrip(
        Counted {
            prefix: narrow,
            item: Number::<u8>::LE,
        },
        &[2, 2, 1],
    );
    map_roundtrip(
        Counted {
            prefix: wide,
            item: pairs,
        },
        &[2, 0, 0, 0, 0, 0, 0, 0, 2, 20, 1, 10],
    );
    set_roundtrip(
        Counted {
            prefix: wide,
            item: Number::<u8>::LE,
        },
        &[2, 0, 0, 0, 0, 0, 0, 0, 2, 1],
    );
}

fn empty_default() {
    let map = OrderedMapCodec(Sequence {
        item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
        count: Count::U32,
    });
    let set = OrderedSetCodec(Sequence {
        item: Number::<u8>::LE,
        count: Count::U32,
    });
    assert_eq!(map.encode(&OrderedMap::new(), 4).unwrap(), [0; 4]);
    assert_eq!(set.encode(&OrderedSet::new(), 4).unwrap(), [0; 4]);
    assert!(map.decode(&[0; 4], LIMITS).unwrap().is_empty());
    assert!(set.decode(&[0; 4], LIMITS).unwrap().is_empty());
}

/// Sentinel policies and exact-exhaustion leniency remain explicit choices.
pub fn termination_strategies() {
    for strategy in [
        Termination::Required,
        Termination::Optional,
        Termination::Omitted,
    ] {
        let pairs = Tuple(Number::<u8>::LE, Number::<u8>::LE);
        let map = ItemSentinel::new(pairs, &[0], strategy, 2).unwrap();
        let set = ItemSentinel::new(Number::<u8>::LE, &[0], strategy, 1).unwrap();
        let omitted = strategy == Termination::Omitted;
        map_roundtrip(
            map,
            if omitted {
                &[2, 20, 1, 10]
            } else {
                &[2, 20, 1, 10, 0]
            },
        );
        set_roundtrip(set, if omitted { &[2, 1] } else { &[2, 1, 0] });
    }

    optional_count();
}

fn optional_count() {
    let map = OrderedMapCodec(Counted {
        prefix: OptionalCount(Number::<u32>::LE),
        item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
    });
    let set = OrderedSetCodec(Counted {
        prefix: OptionalCount(Number::<u32>::LE),
        item: Number::<u8>::LE,
    });
    assert!(map.decode(&[], LIMITS).unwrap().is_empty());
    assert!(set.decode(&[], LIMITS).unwrap().is_empty());
    assert_eq!(map.decode(&[0], LIMITS), Err(Error::Truncated));
    assert_eq!(set.decode(&[0], LIMITS), Err(Error::Truncated));
}
