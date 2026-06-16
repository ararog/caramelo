use crate::Matcher;

/// Creates a matcher that matches values that satisfy any of the given matchers
///
/// # Examples
///
/// ```
/// use caramelo::{or, expect};
/// use caramelo::matchers::{contains};
///
/// expect("hello").to(or!(contains("ell"), contains("xyz")));
/// ```
pub fn or<T>(matchers: Vec<Box<dyn Matcher<T>>>) -> Or<T> {
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
/// expect("hello").to(or!(contains("ell"), contains("xyz")));
/// ```
pub struct Or<T> {
    matchers: Vec<Box<dyn Matcher<T>>>,
}

impl<T> Matcher<T> for Or<T> {
    fn matches(&self, value: &T) -> bool {
        self.matchers
            .iter()
            .any(|m| m.matches(value))
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
            .join(" or ")
    }
}
