use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};

/// Matcher that checks if a Result is Ok
pub fn ok<T, E>() -> impl TypedMatcher<Result<T, E>> {
    IsOk
}

/// Matcher that checks if a Result is Ok
pub struct IsOk;

impl<T, E> Matcher<Result<T, E>> for IsOk {
    fn matches(&self, value: &Result<T, E>) -> bool {
        value.is_ok()
    }

    fn description(&self) -> String {
        "ok".to_string()
    }
}

impl<T, E> TypedMatcher<Result<T, E>> for IsOk {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

/// Matcher that checks if a Result is Err
pub fn err<T, E>() -> impl TypedMatcher<Result<T, E>> {
    IsErr
}

/// Matcher that checks if a Result is Err
pub struct IsErr;

impl<T, E> Matcher<Result<T, E>> for IsErr {
    fn matches(&self, value: &Result<T, E>) -> bool {
        value.is_err()
    }

    fn description(&self) -> String {
        "err".to_string()
    }
}

impl<T, E> TypedMatcher<Result<T, E>> for IsErr {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
