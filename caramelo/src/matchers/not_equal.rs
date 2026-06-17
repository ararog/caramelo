use crate::{
    MatchType::{self, To},
    Matcher, TypedMatch,
};
use std::fmt::Debug;

/// Creates a matcher that matches values not equal to the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::ne;
///
/// expect(5).to(ne(3));
/// ```
pub fn ne<T>(value: T) -> NotEqual<T> {
    NotEqual(value)
}

/// Matcher that matches values not equal to the given value
pub struct NotEqual<T>(T);

impl<T> Matcher<T> for NotEqual<T>
where
    T: PartialEq + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 != *value
    }

    fn description(&self) -> String {
        format!("not equal to {:?}", self.0)
    }
}

impl<T> TypedMatch<T> for NotEqual<T>
where
    T: PartialEq + Debug,
{
    fn matcher_type(&self) -> MatchType {
        To
    }
}
