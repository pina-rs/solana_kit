//! UTC syntax and calendar validation use caller data without consulting wall time.

const MAX_FRACTION_DIGITS: usize = 9;
const LAST_HOUR: u32 = 23;
const LAST_MINUTE_OR_SECOND: u32 = 59;
const DAYS_PER_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_PERIOD: u32 = 4;
const CENTURY: u32 = 100;
const LEAP_CENTURY: u32 = 400;

const YEAR_WIDTH: usize = 4;
const PAIR_WIDTH: usize = 2;

pub(super) fn valid(value: &str) -> bool {
    parse(value).is_some()
}

fn parse(value: &str) -> Option<()> {
    let (date, time) = value.strip_suffix('Z')?.split_once('T')?;
    let [year, month, day] = triple(date, '-', [YEAR_WIDTH, PAIR_WIDTH, PAIR_WIDTH])?;
    let [hour, minute, second] = time_parts(time)?;

    if !valid_date(year, month, day)
        || hour > LAST_HOUR
        || minute > LAST_MINUTE_OR_SECOND
        || second > LAST_MINUTE_OR_SECOND
    {
        return None;
    }

    Some(())
}

fn triple(value: &str, separator: char, widths: [usize; 3]) -> Option<[u32; 3]> {
    let mut fields = value.split(separator);
    let mut output = [0; 3];

    for (slot, width) in output.iter_mut().zip(widths) {
        let field = fields.next()?;

        if field.len() != width || !field.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }

        *slot = field.parse().ok()?;
    }

    fields.next().is_none().then_some(output)
}

fn time_parts(value: &str) -> Option<[u32; 3]> {
    let whole = match value.split_once('.') {
        Some((whole, fraction)) => {
            if fraction.is_empty()
                || fraction.len() > MAX_FRACTION_DIGITS
                || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            {
                return None;
            }

            whole
        }
        None => value,
    };

    triple(whole, ':', [PAIR_WIDTH; 3])
}

fn valid_date(year: u32, month: u32, day: u32) -> bool {
    let Some(index) = month
        .checked_sub(1)
        .and_then(|index| usize::try_from(index).ok())
    else {
        return false;
    };
    let Some(days) = DAYS_PER_MONTH.get(index) else {
        return false;
    };
    let leap = year % LEAP_PERIOD == 0 && (year % CENTURY != 0 || year % LEAP_CENTURY == 0);
    let days = *days + u32::from(month == 2 && leap);
    year != 0 && (1..=days).contains(&day)
}
