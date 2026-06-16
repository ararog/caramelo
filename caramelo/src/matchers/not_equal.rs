use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values not equal to the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::ne;
///
/// expect(5).to_be(ne(3));
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

    fn matcher_type(&self) -> crate::MatchType {
        crate::MatchType::ToBe
    }

    fn description(&self) -> String {
        format!("not equal to {:?}", self.0)
    }
}
