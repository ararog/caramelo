use crate::{MatchType::ToBe, Matcher, TypedMatcher};

/// Creates a matcher that matches truthy values
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::truthy;
///
/// expect(true).to_be(truthy());
/// ```
pub fn truthy() -> Truthy {
    Truthy
}

/// Matcher that matches truthy values
pub struct Truthy;

impl Matcher<bool> for Truthy {
    fn matches(&self, value: &bool) -> bool {
        *value
    }

    fn description(&self) -> String {
        "to be truthy".to_string()
    }
}

impl TypedMatcher<bool> for Truthy {
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}

/// Creates a matcher that matches falsy values
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::falsy;
///
/// expect(false).to_be(falsy());
/// ```
pub fn falsy() -> Falsy {
    Falsy
}

/// Matcher that matches falsy values
pub struct Falsy;

impl Matcher<bool> for Falsy {
    fn matches(&self, value: &bool) -> bool {
        !value
    }

    fn description(&self) -> String {
        "to be falsy".to_string()
    }
}

impl TypedMatcher<bool> for Falsy {
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}
