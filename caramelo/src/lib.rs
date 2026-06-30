#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::matchers::{and, or, And, Or};
use std::fmt::Debug;

/// Module containing matchers for assertions
pub mod matchers;

/// Module containing request wrapper
#[cfg(feature = "http")]
pub mod http;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Default)]
/// Enum representing the type of match operation
pub enum MatchType {
    /// Represents "to" match operation
    #[default]
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

/// Trait for matchers used in assertions
pub trait Matcher<T> {
    /// Checks if the matcher matches the given value
    fn matches(&self, value: &T) -> bool;
    /// Returns a description of the matcher
    fn description(&self) -> String;
}

/// Trait for matchers that have a specific type
pub trait TypedMatcher<T>: Matcher<T> {
    /// Returns the type of the matcher
    fn matcher_type(&self) -> MatchType;
}

/// Trait for extending matchers with additional methods
pub trait MatcherExt<T>: TypedMatcher<T> + Sized + 'static {
    /// Combines this matcher with another using AND logic
    fn and<M: TypedMatcher<T> + 'static>(self, matcher: M) -> And<T> {
        and(vec![Box::new(self), Box::new(matcher)])
    }

    /// Combines this matcher with another using OR logic
    fn or<M: TypedMatcher<T> + 'static>(self, matcher: M) -> Or<T> {
        or(vec![Box::new(self), Box::new(matcher)])
    }
}

/// Implementation of MatcherExt for any type that implements TypedMatcher
impl<M, T> MatcherExt<T> for M where M: TypedMatcher<T> + 'static {}

/// Main expectation struct for assertions
pub struct Expect<T> {
    value: T,
}

impl<T> Expect<T>
where
    T: Debug,
{
    /// Creates a new expectation with the given value
    pub fn new(value: T) -> Self {
        Expect { value }
    }

    fn assert<M>(&mut self, matcher: M, match_type: MatchType)
    where
        M: Matcher<T>,
    {
        if !matcher.matches(&self.value) {
            panic!(
                "Expected {:?} {} {}",
                self.value,
                match_type.description(),
                matcher.description()
            );
        }
    }

    /// Asserts that the value matches the given matcher using "to" syntax
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::{expect, matchers::contains};
    ///
    /// expect("hello").to(contains("ell"));
    /// ```
    pub fn to<M>(mut self, matcher: M) -> Self
    where
        M: TypedMatcher<T>,
    {
        if TypedMatcher::<T>::matcher_type(&matcher) == MatchType::To {
            self.assert(matcher, MatchType::To);
        } else {
            panic!("Matcher must be a 'to' matcher");
        }
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
        M: TypedMatcher<T>,
    {
        if TypedMatcher::<T>::matcher_type(&matcher) == MatchType::ToBe {
            self.assert(matcher, MatchType::ToBe);
        } else {
            panic!("Matcher must be a 'to be' matcher");
        }
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
        M: TypedMatcher<T>,
    {
        if TypedMatcher::<T>::matcher_type(&matcher) == MatchType::ToHave {
            self.assert(matcher, MatchType::ToHave);
        } else {
            panic!("Matcher must be a 'to have' matcher");
        }
        self
    }

    /// Asserts that the value matches the given matcher using the matcher's own syntax
    ///
    /// # Examples
    ///
    /// ```
    /// use caramelo::{expect, matchers::length};
    ///
    /// expect(vec![1, 2, 3]).to_match(length(3));
    /// ```
    pub fn to_match<M>(mut self, matcher: M) -> Self
    where
        M: TypedMatcher<T>,
    {
        let matcher_type = TypedMatcher::<T>::matcher_type(&matcher);
        self.assert(matcher, matcher_type);
        self
    }
}

/// Function to create a new expectation
pub fn expect<T: Debug>(value: T) -> Expect<T> {
    Expect::new(value)
}

/// Main expectation struct for assertions
pub struct Verify<T> {
    value: T,
}

impl<T> Verify<T>
where
    T: Debug,
{
    /// Creates a new expectation with the given value
    pub fn new(value: T) -> Self {
        Verify { value }
    }

    /// Asserts that the value matches the given matcher
    pub fn assert<M>(&mut self, matcher: M) -> bool
    where
        M: Matcher<T>,
    {
        matcher.matches(&self.value)
    }
}

/// Function to create a new expectation
pub fn verify<T: Debug>(value: T) -> Verify<T> {
    Verify::new(value)
}

#[macro_export]
/// Creates an AND matcher that combines multiple matchers
///
/// # Examples
///
/// ```
/// use caramelo::{expect, and, matchers::{eq, gt}};
///
/// expect(5).to_match(and!(eq(5), gt(3)));
/// ```
macro_rules! and {
    ($($matcher:expr),*) => {
        $crate::matchers::and(vec![$(Box::new($matcher)),*])
    };
}

#[macro_export]
/// Creates an OR matcher that combines multiple matchers
///
/// # Examples
///
/// ```
/// use caramelo::{expect, or, matchers::{eq, gt}};
///
/// expect(5).to_match(or!(eq(5), gt(10)));
/// ```
macro_rules! or {
    ($($matcher:expr),*) => {
        $crate::matchers::or(vec![$(Box::new($matcher)),*])
    };
}
