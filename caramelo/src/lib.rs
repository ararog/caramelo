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
    /// Represents "and" match operation
    And,
}

impl MatchType {
    fn description(&self) -> &str {
        match self {
            MatchType::To => "to",
            MatchType::ToBe => "to be",
            MatchType::ToHave => "to have",
            MatchType::And => "",
        }
    }
}

/// Main expectation struct for assertions
pub struct Expect<T> {
    value: T,
    matches: Vec<String>,
}

impl<T> Expect<T>
where
    T: Debug,
{
    /// Creates a new expectation with the given value
    pub fn new(value: T) -> Self {
        Expect { value, matches: Vec::new() }
    }

    fn assert<M>(&mut self, matcher: M, match_type: MatchType)
    where
        M: Matcher<T>,
    {
        if !matcher.matches(&self.value) {
            if self
                .matches
                .is_empty()
            {
                panic!(
                    "Expected {:?} {} {}",
                    self.value,
                    match_type.description(),
                    matcher.description()
                );
            } else {
                let mut message = Vec::new();

                self.matches.push(
                    matcher
                        .description()
                        .to_string(),
                );

                for match_str in &self.matches {
                    message.push(match_str.clone());
                }

                panic!("Expected {}", message.join(" and "));
            }
        } else {
            if self
                .matches
                .is_empty()
            {
                self.matches
                    .push(format!(
                        "{:?} {} {}",
                        self.value,
                        match_type.description(),
                        matcher.description()
                    ));
            } else {
                self.matches.push(
                    matcher
                        .description()
                        .to_string(),
                );
            }
        }
    }

    /// Asserts that the value matches the given matcher using "to" syntax
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::{expect, matchers::eq};
    ///
    /// expect(5).to(eq(5));
    /// ```
    pub fn to<M>(mut self, matcher: M) -> Self
    where
        M: Matcher<T>,
    {
        self.assert(matcher, MatchType::To);
        self
    }

    /// Asserts that the value matches the given matcher using "to be" syntax
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::{expect, matchers::eq};
    ///
    /// expect(5).to_be(eq(5));
    /// ```
    pub fn to_be<M>(mut self, matcher: M) -> Self
    where
        M: Matcher<T>,
    {
        self.assert(matcher, MatchType::ToBe);
        self
    }

    /// Asserts that the value matches the given matcher using "to have" syntax
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::{expect, matchers::length};
    ///
    /// expect(vec![1, 2, 3]).to_have(length(3));
    /// ```
    pub fn to_have<M>(mut self, matcher: M) -> Self
    where
        M: Matcher<T>,
    {
        self.assert(matcher, MatchType::ToHave);
        self
    }

    /// Asserts that the value matches the given matcher using "and" syntax
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::{expect, matchers::{eq, gt}};
    ///
    /// expect(5).to_be(eq(5)).and(gt(0));
    /// ```
    pub fn and<M>(mut self, matcher: M) -> Self
    where
        M: Matcher<T>,
    {
        self.assert(matcher, MatchType::And);
        self
    }
}

/// Function to create a new expectation
pub fn expect<T: Debug>(value: T) -> Expect<T> {
    Expect::new(value)
}

/// Trait for matchers used in assertions
pub trait Matcher<T> {
    /// Checks if the matcher matches the given value
    fn matches(&self, value: &T) -> bool;
    /// Returns a description of the matcher
    fn description(&self) -> String;
}
