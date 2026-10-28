//! Closed token routes keep arbitrary query data out of path selection.

/// A Token API v2 category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// Highest organic score.
    TopOrganicScore,
    /// Most traded tokens.
    TopTraded,
    /// Trending tokens.
    TopTrending,
}

impl Category {
    pub(crate) const fn segment(self) -> &'static str {
        match self {
            Self::TopOrganicScore => "toporganicscore",
            Self::TopTraded => "toptraded",
            Self::TopTrending => "toptrending",
        }
    }
}

/// A Token API v2 category interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interval {
    /// Five minutes.
    FiveMinutes,
    /// One hour.
    OneHour,
    /// Six hours.
    SixHours,
    /// Twenty-four hours, the pinned reference default.
    #[default]
    OneDay,
}

impl Interval {
    pub(crate) const fn segment(self) -> &'static str {
        match self {
            Self::FiveMinutes => "5m",
            Self::OneHour => "1h",
            Self::SixHours => "6h",
            Self::OneDay => "24h",
        }
    }
}

/// A token category page limit in the inclusive range 1 through 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenLimit(u8);

impl TokenLimit {
    /// Returns a valid limit, or `None` for zero and values above 100.
    pub const fn new(limit: u8) -> Option<Self> {
        if limit == 0 || limit > 100 {
            return None;
        }

        Some(Self(limit))
    }

    /// Borrows the validated numeric limit.
    pub const fn get(self) -> u8 {
        self.0
    }
}
