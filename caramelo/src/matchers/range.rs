use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};
use std::fmt::Debug;

/// Creates a matcher that matches values between min and max (exclusive)
/// It is the same as 1..10 in Rust
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::in_exc;
///
/// expect(5).to_be(in_exc(1, 10));
/// ```
pub fn in_exc<T>(min: T, max: T) -> Between<T> {
    Between { min, max, inclusive: false }
}

/// Creates a matcher that matches values between min and max (inclusive)
/// It is the same as 1..=10 in Rust
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::in_inc;
///
/// expect(5).to_be(in_inc(1, 10));
/// ```
pub fn in_inc<T>(min: T, max: T) -> Between<T> {
    Between { min, max, inclusive: true }
}

/// Matcher that matches values between min and max (inclusive or exclusive)
pub struct Between<T> {
    min: T,
    max: T,
    inclusive: bool,
}

impl<T> Matcher<T> for Between<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        if self.inclusive {
            value >= &self.min && value <= &self.max
        } else {
            value > &self.min && value < &self.max
        }
    }

    fn description(&self) -> String {
        if self.inclusive {
            format!("between {:?} and {:?}", self.min, self.max)
        } else {
            format!("between {:?} and {:?} (exclusive)", self.min, self.max)
        }
    }
}

impl<T> TypedMatcher<T> for Between<T>
where
    T: PartialOrd + Debug,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
