use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};
use std::fmt::Debug;

/// Creates a matcher that matches values equal to the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::eq;
///
/// expect(5).to_be(eq(5));
/// ```
pub fn eq<T>(value: T) -> Equal<T> {
    Equal(value)
}

/// Matcher that matches values equal to the given value
pub struct Equal<A>(A);

impl<T> Matcher<T> for Equal<T>
where
    T: PartialEq + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

impl<T> TypedMatcher<T> for Equal<T>
where
    T: PartialEq + Debug,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

impl Matcher<String> for Equal<&str> {
    fn matches(&self, value: &String) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

impl TypedMatcher<String> for Equal<&str> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

impl Matcher<&String> for Equal<&str> {
    fn matches(&self, value: &&String) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

impl TypedMatcher<&String> for Equal<&str> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
