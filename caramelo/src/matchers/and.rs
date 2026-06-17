use crate::{
    MatchType::{self, To},
    Matcher, TypedMatch,
};

/// Creates a matcher that matches values that satisfy all given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to_self(and!(contains("ell"), contains("llo")));
/// ```
pub fn and<T>(matchers: Vec<Box<dyn TypedMatch<T>>>) -> And<T> {
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
/// expect("hello").to_self(and!(contains("ell"), contains("llo")));
/// ```
pub struct And<T> {
    matchers: Vec<Box<dyn TypedMatch<T>>>,
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

impl<T> TypedMatch<T> for And<T>
where
    T: 'static,
{
    fn matcher_type(&self) -> MatchType {
        self.matchers
            .first()
            .map(|m| TypedMatch::matcher_type(m.as_ref()))
            .unwrap_or(To)
    }
}
