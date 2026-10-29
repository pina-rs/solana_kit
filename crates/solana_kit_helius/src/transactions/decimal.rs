//! Floors exact JSON decimals only as far as a caller's comparison ceiling.

use crate::Decimal;

/// Clamps negative values to zero and returns min(floor(value), ceiling) without a float.
pub(super) fn floor_at_most(value: &Decimal, ceiling: u128) -> u128 {
    let number = value.number().as_str();
    let (significand, exponent) = number.split_once(['e', 'E']).unwrap_or((number, "0"));

    if number.starts_with('-') || significand.trim_matches(['0', '.']).is_empty() {
        return 0;
    }

    let negative_exponent = exponent.starts_with('-');
    let magnitude = exponent.trim_start_matches(['+', '-']).parse::<usize>();
    let integer_digits = significand.find('.').unwrap_or(significand.len());

    match magnitude {
        Ok(magnitude) if negative_exponent => floor_digits(
            significand,
            integer_digits.saturating_sub(magnitude),
            ceiling,
        ),
        Ok(magnitude) => floor_digits(
            significand,
            integer_digits.saturating_add(magnitude),
            ceiling,
        ),
        // Decimal guarantees valid JSON. An unrepresentable exponent exceeds
        // the token's addressable digit count, so only its direction matters.
        Err(_) if negative_exponent => 0,
        Err(_) => ceiling,
    }
}

/// Stops at the ceiling; enormous positive exponents add at most u128's digit width.
fn floor_digits(significand: &str, count: usize, ceiling: u128) -> u128 {
    let digits = significand
        .bytes()
        .filter(u8::is_ascii_digit)
        .chain(core::iter::repeat(b'0'))
        .take(count);
    let mut whole = 0_u128;

    for digit in digits {
        let Some(next) = whole
            .checked_mul(10)
            .and_then(|whole| whole.checked_add(u128::from(digit - b'0')))
        else {
            return ceiling;
        };

        if next >= ceiling {
            return ceiling;
        }

        whole = next;
    }

    whole
}
