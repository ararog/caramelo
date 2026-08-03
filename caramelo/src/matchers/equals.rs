use crate::{
    MatchType::{self, ToBe},
    Matcher, TypedMatcher,
};
#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "http")]
use http::StatusCode;
use std::fmt::Debug;

/// Creates a matcher that matches values equal to the given value
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::eq;
///
/// expect(5).to_be(eq(5));
/// ```
pub fn eq<T>(value: T) -> Equal<T> {
    Equal(value)
}

/// Matcher that matches values equal to the given value
pub struct Equal<T>(T);

impl<T> Matcher<T> for Equal<T>
where
    T: PartialEq + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

impl<T> TypedMatcher<T> for Equal<T>
where
    T: PartialEq + Debug,
{
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

impl Matcher<String> for Equal<&str> {
    fn matches(&self, value: &String) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

impl TypedMatcher<String> for Equal<&str> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

impl Matcher<&String> for Equal<&str> {
    fn matches(&self, value: &&String) -> bool {
        self.0 == *value
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

impl TypedMatcher<&String> for Equal<&str> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

#[cfg(feature = "bytes")]
impl Matcher<Bytes> for Equal<&[u8]> {
    fn matches(&self, value: &Bytes) -> bool {
        value == self.0
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

#[cfg(feature = "bytes")]
impl TypedMatcher<Bytes> for Equal<&[u8]> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

#[cfg(feature = "http")]
impl Matcher<StatusCode> for Equal<u16> {
    fn matches(&self, value: &StatusCode) -> bool {
        value == &self.0
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

#[cfg(feature = "http")]
impl TypedMatcher<StatusCode> for Equal<u16> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}

#[cfg(feature = "http")]
impl Matcher<u16> for Equal<StatusCode> {
    fn matches(&self, value: &u16) -> bool {
        value == &self.0
    }

    fn description(&self) -> String {
        format!("equals to {:?}", self.0)
    }
}

#[cfg(feature = "http")]
impl TypedMatcher<u16> for Equal<StatusCode> {
    fn matcher_type(&self) -> MatchType {
        ToBe
    }
}
