use crate::{
    MatchType::{self, To},
    Matcher, TypedMatcher,
};

/// Creates a matcher that matches values that satisfy all given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(and!(contains("ell"), contains("llo")));
/// ```
pub fn and<T>(matchers: Vec<Box<dyn TypedMatcher<T>>>) -> And<T> {
    And { matchers }
}

/// Matcher that combines multiple matchers with AND logic
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(and!(contains("ell"), contains("llo")));
/// ```
pub struct And<T> {
    matchers: Vec<Box<dyn TypedMatcher<T>>>,
}

impl<T> And<T> {
    /// Creates a new And matcher with the given matchers
    pub fn new(matchers: Vec<Box<dyn TypedMatcher<T>>>) -> Self {
        And { matchers }
    }
}

impl<T> Matcher<T> for And<T>
where
    T: 'static,
{
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .all(|m| m.matches(value))
    }

    fn description(&self) -> String {
        self.matchers
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join(" and ")
    }
}

impl<T> TypedMatcher<T> for And<T>
where
    T: 'static,
{
    fn matcher_type(&self) -> MatchType {
        self.matchers
            .first()
            .map(|m| TypedMatcher::matcher_type(m.as_ref()))
            .unwrap_or(To)
    }
}
