use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values between min and max (exclusive)
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::in_range_to;
///
/// expect(5).to_be(in_range_to(1, 10));
/// ```
pub fn in_range_to<T>(min: T, max: T) -> Between<T> {
    Between { min, max, inclusive: false }
}

/// Creates a matcher that matches values between min and max (inclusive)
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::in_range_inc;
///
/// expect(5).to_be(in_range_inc(1, 10));
/// ```
pub fn in_range_inc<T>(min: T, max: T) -> Between<T> {
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
