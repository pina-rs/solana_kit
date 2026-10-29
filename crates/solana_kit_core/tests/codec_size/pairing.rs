use alloc::{rc::Rc, vec};
use core::{cell::Cell, error::Error as _};
use solana_kit_core::{
    codecs::{
        Bytes, Decoder, Encoder, Error, Limits, Mapped, Number, PairError, Paired, Reader,
        ShortU16, Size, Unit, Writer,
    },
    errors::{ContextValue, Diagnostic, ErrorFacts},
};

const LIMITS: Limits = Limits::new(64, 64);

#[path = "observers.rs"]
mod observers;

/// Counts every operation that constructor rejection must leave untouched.
#[derive(Default)]
struct Calls {
    queries: Cell<usize>,
    sizes: Cell<usize>,
    writes: Cell<usize>,
    reads: Cell<usize>,
}

/// An application-owned byte codec with an independently selected declaration.
struct Declared {
    declaration: Result<Size, Error>,
    calls: Rc<Calls>,
}

impl Declared {
    fn size(&self) -> Result<Size, Error> {
        self.calls.queries.set(self.calls.queries.get() + 1);
        self.declaration
    }
}

impl Encoder<u8> for Declared {
    fn encode_size(&self) -> Result<Size, Error> {
        self.size()
    }

    fn encoded_len(&self, value: &u8) -> Result<usize, Error> {
        self.calls.sizes.set(self.calls.sizes.get() + 1);
        Number::<u8>::LE.encoded_len(value)
    }

    fn write(&self, value: &u8, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.calls.writes.set(self.calls.writes.get() + 1);
        Number::<u8>::LE.write(value, writer)
    }
}

impl Decoder<u8> for Declared {
    fn decode_size(&self) -> Result<Size, Error> {
        self.size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<u8, Error> {
        self.calls.reads.set(self.calls.reads.get() + 1);
        Number::<u8>::LE.read(reader)
    }
}

/// Checks the public constructor and confirms that only its two queries ran.
fn pair(encoder: Size, decoder: Size) -> Result<(), PairError> {
    let calls = Rc::new(Calls::default());
    let result = Paired::try_new(
        Declared {
            declaration: Ok(encoder),
            calls: Rc::clone(&calls),
        },
        Declared {
            declaration: Ok(decoder),
            calls: Rc::clone(&calls),
        },
    );
    assert_eq!(calls.queries.get(), 2);
    assert_eq!(calls.sizes.get(), 0);
    assert_eq!(calls.writes.get(), 0);
    assert_eq!(calls.reads.get(), 0);
    result.map(|_| ())
}

fn matching_declarations() {
    use Size::{Fixed, Variable};

    for size in [
        Fixed(0),
        Fixed(1),
        Fixed(4),
        Variable { max: None },
        Variable { max: Some(0) },
        Variable { max: Some(3) },
    ] {
        assert_eq!(pair(size, size), Ok(()));
    }

    assert_eq!(
        Paired::try_new(Unit, Unit).unwrap().encode(&(), 0),
        Ok(vec![])
    );
    assert!(Paired::<_, _, [u8], alloc::vec::Vec<u8>>::try_new(Bytes, Bytes).is_ok());
    assert!(Paired::try_new(ShortU16, ShortU16).is_ok());
}

/// Fixed, variable, unknown, zero and optional maxima retain distinct meanings.
pub fn constructor_matrix() {
    use Size::{Fixed, Unknown, Variable};

    matching_declarations();

    assert!(matches!(
        Paired::try_new(Number::<u8>::LE, Number::<u32>::LE),
        Err(PairError::FixedSizeMismatch {
            encoder: 1,
            decoder: 4
        })
    ));
    assert!(matches!(
        Paired::try_new(Number::<u16>::LE, ShortU16),
        Err(PairError::KindMismatch)
    ));

    for size in [Unknown, Fixed(0), Variable { max: None }] {
        assert_eq!(pair(Unknown, size), Err(PairError::UnknownSize));
        assert_eq!(pair(size, Unknown), Err(PairError::UnknownSize));
    }

    assert_eq!(
        pair(Fixed(1), Fixed(4)),
        Err(PairError::FixedSizeMismatch {
            encoder: 1,
            decoder: 4
        })
    );
    assert_eq!(
        pair(Fixed(4), Fixed(1)),
        Err(PairError::FixedSizeMismatch {
            encoder: 4,
            decoder: 1
        })
    );

    variable_mismatches();
}

fn variable_mismatches() {
    use Size::{Fixed, Variable};

    for max in [None, Some(0), Some(4)] {
        assert_eq!(
            pair(Fixed(0), Variable { max }),
            Err(PairError::KindMismatch)
        );
        assert_eq!(
            pair(Variable { max }, Fixed(0)),
            Err(PairError::KindMismatch)
        );
    }

    for (encoder, decoder) in [
        (None, Some(0)),
        (Some(0), None),
        (Some(0), Some(1)),
        (Some(3), Some(4)),
        (Some(3), None),
    ] {
        assert_eq!(
            pair(Variable { max: encoder }, Variable { max: decoder }),
            Err(PairError::MaxSizeMismatch { encoder, decoder })
        );
    }
}

/// Mapping and observer queries leave callbacks untouched, even during rejection.
pub fn local_callbacks() {
    observers::metadata_and_callbacks();

    let writes = Rc::new(Cell::new(0));
    let reads = Rc::new(Cell::new(0));
    let write_calls = Rc::clone(&writes);
    let read_calls = Rc::clone(&reads);

    let mapped = Mapped::new(
        Number::<u8>::LE,
        move |value: &u16| {
            write_calls.set(write_calls.get() + 1);
            u8::try_from(*value).map_err(|_| Error::Overflow)
        },
        move |value: u8| {
            read_calls.set(read_calls.get() + 1);
            Ok(u16::from(value))
        },
    );

    assert_eq!(mapped.encode_size(), Ok(Size::Fixed(1)));
    assert_eq!(mapped.decode_size(), Ok(Size::Fixed(1)));
    assert!(matches!(
        Paired::try_new(&mapped, Number::<u32>::LE),
        Err(PairError::FixedSizeMismatch {
            encoder: 1,
            decoder: 4
        })
    ));
    assert_eq!((writes.get(), reads.get()), (0, 0));

    let paired = Paired::try_new(mapped, Number::<u8>::LE).unwrap();
    assert_eq!(paired.encode_size(), Ok(Size::Fixed(1)));
    assert_eq!(paired.decode_size(), Ok(Size::Fixed(1)));
    assert_eq!((writes.get(), reads.get()), (0, 0));
    assert_eq!(paired.encode(&7u16, 1), Ok(vec![7]));
    assert_eq!(paired.decode_prefix(&[99, 7, 88], 1, LIMITS), Ok((7u8, 2)));
    assert_eq!(paired.decode(&[7, 88], LIMITS), Err(Error::TrailingBytes));
    assert_eq!((writes.get(), reads.get()), (2, 0));

    let (mapped, _) = paired.into_parts();
    assert_eq!(mapped.decode(&[7], LIMITS), Ok(7u16));
    assert_eq!(reads.get(), 1);
}

/// The retained facts boundary distinguishes null, zero and an absent key.
pub fn error_facts() {
    let kind = pair(Size::Fixed(0), Size::Variable { max: Some(0) }).unwrap_err();
    assert_eq!(kind.error_code().unwrap().value(), 8078004);
    assert_eq!(kind.error_context().iter().count(), 0);
    let fixed = pair(Size::Fixed(1), Size::Fixed(4)).unwrap_err();
    assert_eq!(fixed.error_code().unwrap().value(), 8078005);
    let facts = fixed.error_context();
    assert_eq!(
        facts.get("encoderFixedSize"),
        Some(&ContextValue::Unsigned(1))
    );
    assert_eq!(
        facts.get("decoderFixedSize"),
        Some(&ContextValue::Unsigned(4))
    );

    for (encoder, decoder) in [
        (None, Some(0)),
        (Some(0), None),
        (Some(3), Some(4)),
        (Some(0), Some(4)),
    ] {
        maximum_error_facts(encoder, decoder);
    }

    assert_eq!(PairError::UnknownSize.error_code(), None);
    assert!(PairError::UnknownSize.source().is_none());
    metadata_cause();
}

fn maximum_error_facts(encoder: Option<usize>, decoder: Option<usize>) {
    let error = pair(
        Size::Variable { max: encoder },
        Size::Variable { max: decoder },
    )
    .unwrap_err();
    let facts = error.error_context();
    assert_eq!(error.error_code().unwrap().value(), 8078006);
    assert_eq!(facts.iter().count(), 2);
    assert_eq!(
        facts.get("encoderMaxSize"),
        Some(&encoder.map_or(ContextValue::Null, ContextValue::from))
    );
    assert_eq!(
        facts.get("decoderMaxSize"),
        Some(&decoder.map_or(ContextValue::Null, ContextValue::from))
    );
    let normalized = Diagnostic::from_context(error.error_code().unwrap(), facts);
    let retained = if encoder.is_none() || decoder.is_none() {
        1
    } else {
        2
    };

    assert_eq!(normalized.context().iter().count(), retained);
    assert!(error.source().is_none());
}

/// Query failures retain their exact original source and available facts.
fn metadata_cause() {
    for original in [Error::Overflow, Error::InvalidBoolean { value: 2 }] {
        let calls = Rc::new(Calls::default());
        let error = Paired::try_new(
            Declared {
                declaration: Err(original),
                calls: Rc::clone(&calls),
            },
            Number::<u8>::LE,
        )
        .err()
        .unwrap();

        assert_eq!(error, PairError::Metadata(original));
        assert_eq!(
            error.source().unwrap().downcast_ref::<Error>(),
            Some(&original)
        );
        assert_eq!(error.error_code(), original.error_code());
        assert_eq!(error.error_context(), original.error_context());
        assert_eq!(calls.queries.get(), 1);
        assert_eq!(calls.sizes.get(), 0);
        assert_eq!(calls.writes.get(), 0);
        assert_eq!(calls.reads.get(), 0);
    }

    let calls = Rc::new(Calls::default());
    let error = Paired::try_new(
        Number::<u8>::LE,
        Declared {
            declaration: Err(Error::Truncated),
            calls: Rc::clone(&calls),
        },
    )
    .err()
    .unwrap();
    assert_eq!(error, PairError::Metadata(Error::Truncated));
    assert_eq!(calls.queries.get(), 1);
}
