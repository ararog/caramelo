use crate::{
    MatchType::{self, To},
    Matcher, TypedMatcher,
};

/// Creates a matcher that matches values that satisfy any of the given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{or, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(or!(contains("ell"), contains("xyz")));
/// ```
pub fn or<T>(matchers: Vec<Box<dyn TypedMatcher<T>>>) -> Or<T> {
    Or { matchers }
}

/// Matcher that combines multiple matchers with OR logic
///
/// # Examples
///
/// ```
/// use caramelo::{or, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_match(or!(contains("ell"), contains("xyz")));
/// ```
pub struct Or<T> {
    matchers: Vec<Box<dyn TypedMatcher<T>>>,
}

impl<T> Or<T> {
    /// Creates a new Or matcher with the given matchers
    pub fn new(matchers: Vec<Box<dyn TypedMatcher<T>>>) -> Self {
        Or { matchers }
    }
}

impl<T> Matcher<T> for Or<T> {
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .any(|m| m.matches(value))
    }

    fn description(&self) -> String {
        self.matchers
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join(" or ")
    }
}

impl<T> TypedMatcher<T> for Or<T> {
    fn matcher_type(&self) -> MatchType {
        self.matchers
            .first()
            .map(|m| TypedMatcher::<T>::matcher_type(m.as_ref()))
            .unwrap_or(To)
    }
}
