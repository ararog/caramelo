use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};

/// Matcher that checks if an Option is Some
pub fn some<T>() -> impl TypedMatcher<Option<T>> {
    IsSome
}

/// Matcher that checks if an Option is Some
pub struct IsSome;

impl<T> Matcher<Option<T>> for IsSome {
    fn matches(&self, value: &Option<T>) -> bool {
        value.is_some()
    }

    fn description(&self) -> String {
        "some".to_string()
    }
}

impl<T> TypedMatcher<Option<T>> for IsSome {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

/// Matcher that checks if an Option is None
pub fn none<T>() -> impl TypedMatcher<Option<T>> {
    IsNone
}

/// Matcher that checks if an Option is None
pub struct IsNone;

impl<T> Matcher<Option<T>> for IsNone {
    fn matches(&self, value: &Option<T>) -> bool {
        value.is_none()
    }

    fn description(&self) -> String {
        "none".to_string()
    }
}

impl<T> TypedMatcher<Option<T>> for IsNone {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
