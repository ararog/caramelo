use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values less than the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::lt;
///
/// expect(3).to_be(lt(5));
/// ```
pub fn lt<T>(value: T) -> LessThan<T> {
    LessThan(value)
}

/// Matcher that matches values less than the given value
pub struct LessThan<T>(T);

impl<T> Matcher<T> for LessThan<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value < self.0
    }

    fn description(&self) -> String {
        format!("less than {:?}", self.0)
    }
}

/// Creates a matcher that matches values less than or equal to the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::le;
///
/// expect(3).to_be(le(3));
/// ```
pub fn le<T>(value: T) -> LessThanOrEqual<T> {
    LessThanOrEqual(value)
}

/// Matcher that matches values less than or equal to the given value
pub struct LessThanOrEqual<T>(T);

impl<T> Matcher<T> for LessThanOrEqual<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value <= self.0
    }

    fn description(&self) -> String {
        format!("less than or equal to {:?}", self.0)
    }
}
