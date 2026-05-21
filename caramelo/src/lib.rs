#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::fmt::Debug;

/// Module containing assertions
pub mod assertions;

/// Module containing matchers for assertions
pub mod matchers;

#[cfg(test)]
mod tests;

/// Enum representing the type of match operation
pub enum MatchType {
    /// Represents "to" match operation
    To,
    /// Represents "to be" match operation
    ToBe,
    /// Represents "to have" match operation
    ToHave,
}

impl MatchType {
    fn description(&self) -> &str {
        match self {
            MatchType::To => "to",
            MatchType::ToBe => "to be",
            MatchType::ToHave => "to have",
        }
    }
}

/// Main expectation struct for assertions
pub struct Expect<T, M> {
    value: T,
    _matcher: std::marker::PhantomData<M>,
}

impl<T, M> Expect<T, M>
where
    T: Debug,
    M: Matcher<T>,
{
    /// Creates a new expectation with the given value
    pub fn new(value: T) -> Self {
        Expect { value, _matcher: std::marker::PhantomData }
    }

    fn assert(&self, matcher: M, match_type: MatchType) {
        if !matcher.matches(&self.value) {
            panic!(
                "Expected {:?} {} {}",
                self.value,
                match_type.description(),
                matcher.description()
            );
        }
    }

    /// Asserts that the value matches the given matcher using "to be" syntax
    pub fn to(&self, matcher: M) {
        self.assert(matcher, MatchType::To);
    }

    /// Asserts that the value matches the given matcher using "to be" syntax
    pub fn to_be(&self, matcher: M) {
        self.assert(matcher, MatchType::ToBe);
    }

    /// Asserts that the value matches the given matcher using "to have" syntax
    pub fn to_have(&self, matcher: M) {
        self.assert(matcher, MatchType::ToHave);
    }
}

/// Function to create a new expectation
pub fn expect<T: Debug, M: Matcher<T>>(value: T) -> Expect<T, M> {
    Expect::new(value)
}

/// Trait for matchers used in assertions
pub trait Matcher<T> {
    /// Checks if the matcher matches the given value
    fn matches(&self, value: &T) -> bool;
    /// Returns a description of the matcher
    fn description(&self) -> String;
}
