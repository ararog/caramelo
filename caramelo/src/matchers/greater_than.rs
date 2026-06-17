use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};
use std::fmt::Debug;

/// Creates a matcher that matches values greater than the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::gt;
///
/// expect(5).to_be(gt(3));
/// ```
pub fn gt<T>(value: T) -> GreaterThan<T> {
    GreaterThan(value)
}

/// Matcher that matches values greater than the given value
pub struct GreaterThan<T>(T);

impl<T> Matcher<T> for GreaterThan<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value > self.0
    }

    fn description(&self) -> String {
        format!("greater than {:?}", self.0)
    }
}

impl<T> TypedMatcher<T> for GreaterThan<T>
where
    T: PartialOrd + Debug,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

/// Creates a matcher that matches values greater than or equal to the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::ge;
///
/// expect(5).to_be(ge(5));
/// ```
pub fn ge<T>(value: T) -> GreaterThanOrEqual<T> {
    GreaterThanOrEqual(value)
}

/// Matcher that matches values greater than or equal to the given value
pub struct GreaterThanOrEqual<T>(T);

impl<T> Matcher<T> for GreaterThanOrEqual<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value >= self.0
    }

    fn description(&self) -> String {
        format!("greater than or equal to {:?}", self.0)
    }
}

impl<T> TypedMatcher<T> for GreaterThanOrEqual<T>
where
    T: PartialOrd + Debug,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
