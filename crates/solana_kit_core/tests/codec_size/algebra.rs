use solana_kit_core::codecs::{Encoder, Error, PairError, Paired, Size, Unit, Writer};

/// A stack-only declared composition exercises errors through the real boundary.
struct Sum([Size; 3]);

impl Encoder<()> for Sum {
    fn encode_size(&self) -> Result<Size, Error> {
        Size::checked_sum(&self.0)
    }

    fn encoded_len(&self, value: &()) -> Result<usize, Error> {
        Unit.encoded_len(value)
    }

    fn write(&self, value: &(), writer: &mut Writer<'_>) -> Result<(), Error> {
        Unit.write(value, writer)
    }
}

/// Empty and zero identities preserve the declared format class.
pub fn kinds_and_identities() {
    use Size::{Fixed, Unknown, Variable};

    assert_eq!(Size::checked_sum(&[]), Ok(Fixed(0)));
    assert_eq!(Size::choice(&[]), Fixed(0));
    assert_eq!(Size::checked_sum(&[Fixed(0), Fixed(2)]), Ok(Fixed(2)));
    assert_eq!(Size::choice(&[Fixed(2)]), Fixed(2));
    assert_eq!(Size::choice(&[Fixed(2), Fixed(2)]), Fixed(2));
    assert_eq!(
        Size::choice(&[Fixed(2), Fixed(3)]),
        Variable { max: Some(3) }
    );
    assert_eq!(
        Size::choice(&[Fixed(0), Variable { max: Some(0) }]),
        Variable { max: Some(0) }
    );
    assert_eq!(
        Size::choice(&[Fixed(3), Variable { max: None }]),
        Variable { max: None }
    );
    assert_eq!(Size::choice(&[Unknown, Fixed(3)]), Unknown);
    assert_eq!(
        Size::checked_sum(&[Fixed(3), Variable { max: Some(2) }]),
        Ok(Variable { max: Some(5) })
    );
    assert_eq!(
        Size::checked_sum(&[Variable { max: None }, Fixed(3)]),
        Ok(Variable { max: None })
    );
    assert_eq!(Size::checked_sum(&[Unknown, Fixed(3)]), Ok(Unknown));

    for size in [
        Unknown,
        Fixed(0),
        Fixed(3),
        Variable { max: None },
        Variable { max: Some(0) },
    ] {
        assert_eq!(size.checked_repeat(0), Ok(Fixed(0)));
        assert_eq!(size.checked_repeat(1), Ok(size));
    }

    assert_eq!(Fixed(3).checked_repeat(2), Ok(Fixed(6)));
    assert_eq!(
        Variable { max: Some(3) }.checked_repeat(2),
        Ok(Variable { max: Some(6) })
    );
    assert_eq!(Fixed(0).fixed_size(), Some(0));
    assert_eq!(Fixed(2).max_size(), Some(2));
    assert_eq!(Variable { max: Some(0) }.fixed_size(), None);
    assert_eq!(Variable { max: Some(0) }.max_size(), Some(0));
    assert_eq!(Variable { max: None }.max_size(), None);
    assert_eq!(Unknown.max_size(), None);
}

/// Known components cannot hide overflow behind an unknown class or maximum.
pub fn overflow() {
    use Size::{Fixed, Unknown, Variable};

    for unknown in [Unknown, Variable { max: None }] {
        for sizes in [
            [unknown, Fixed(usize::MAX), Fixed(1)],
            [Fixed(usize::MAX), unknown, Fixed(1)],
            [Fixed(usize::MAX), Fixed(1), unknown],
        ] {
            assert_eq!(Size::checked_sum(&sizes), Err(Error::Overflow));
            assert!(matches!(
                Paired::try_new(Sum(sizes), Unit),
                Err(PairError::Metadata(Error::Overflow))
            ));
        }
    }

    assert_eq!(
        Size::checked_sum(&[
            Variable {
                max: Some(usize::MAX)
            },
            Fixed(1)
        ]),
        Err(Error::Overflow)
    );
    assert_eq!(Fixed(2).checked_repeat(usize::MAX), Err(Error::Overflow));
    assert_eq!(
        Variable { max: Some(2) }.checked_repeat(usize::MAX),
        Err(Error::Overflow)
    );
    assert_eq!(
        Size::choice(&[Fixed(usize::MAX), Fixed(1)]),
        Variable {
            max: Some(usize::MAX)
        }
    );
}
