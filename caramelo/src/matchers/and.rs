use crate::Matcher;

/// Creates a matcher that matches values that satisfy all given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{and, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to(and!(contains("ell"), contains("llo")));
/// ```
pub fn and<T>(matchers: Vec<Box<dyn Matcher<T>>>) -> And<T> {
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
/// expect("hello").to(and!(contains("ell"), contains("llo")));
/// ```
pub struct And<T> {
    matchers: Vec<Box<dyn Matcher<T>>>,
}

impl<T> Matcher<T> for And<T> {
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .all(|m| m.matches(value))
    }

    fn matcher_type(&self) -> crate::MatchType {
        self.matchers
            .first()
            .map(|m| m.matcher_type())
            .unwrap_or(crate::MatchType::To)
    }

    fn description(&self) -> String {
        self.matchers
            .iter()
            .map(|m| m.description())
            .collect::<Vec<_>>()
            .join(" and ")
    }
}
