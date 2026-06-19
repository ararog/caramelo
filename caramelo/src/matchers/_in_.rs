use std::{
    collections::{BTreeSet, HashSet},
    fmt::Debug,
};

use crate::{MatchType::ToBe, Matcher, TypedMatcher};

/// Creates a matcher that checks if a value is in a collection.
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::_in_;
///
/// expect("John").to_be(_in_(vec!["John", "Jane"]));
/// ```
pub fn _in_<T>(items: T) -> In<T> {
    In { items }
}

/// A matcher that checks if a value is in a collection.
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::_in_;
///
/// expect("John").to_be(_in_(vec!["John", "Jane"]));
/// ```
pub struct In<T> {
    items: T,
}

impl<T> Matcher<T> for In<Vec<T>>
where
    T: PartialEq + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.items
            .contains(value)
    }

    fn description(&self) -> String {
        format!("in {:?}", self.items)
    }
}

impl<T> TypedMatcher<T> for In<Vec<T>>
where
    T: PartialEq + Debug,
{
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}

impl Matcher<String> for In<Vec<&str>> {
    fn matches(&self, value: &String) -> bool {
        self.items
            .contains(&value.as_str())
    }

    fn description(&self) -> String {
        format!("in {:?}", self.items)
    }
}

impl TypedMatcher<String> for In<Vec<&str>> {
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}

impl<T> Matcher<T> for In<HashSet<T>>
where
    T: Eq + std::hash::Hash + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.items
            .contains(value)
    }

    fn description(&self) -> String {
        format!("in {:?}", self.items)
    }
}

impl<T> TypedMatcher<T> for In<HashSet<T>>
where
    T: Eq + std::hash::Hash + Debug,
{
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}

impl Matcher<String> for In<HashSet<&str>> {
    fn matches(&self, value: &String) -> bool {
        self.items
            .contains(value.as_str())
    }

    fn description(&self) -> String {
        format!("in {:?}", self.items)
    }
}

impl TypedMatcher<String> for In<HashSet<&str>> {
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}

impl<T> Matcher<T> for In<BTreeSet<T>>
where
    T: Eq + std::hash::Hash + Debug + std::cmp::Ord,
{
    fn matches(&self, value: &T) -> bool {
        self.items
            .contains(value)
    }

    fn description(&self) -> String {
        format!("in {:?}", self.items)
    }
}

impl<T> TypedMatcher<T> for In<BTreeSet<T>>
where
    T: Eq + std::hash::Hash + Debug + std::cmp::Ord,
{
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}

impl Matcher<String> for In<BTreeSet<&str>> {
    fn matches(&self, value: &String) -> bool {
        self.items
            .contains(value.as_str())
    }

    fn description(&self) -> String {
        format!("in {:?}", self.items)
    }
}

impl TypedMatcher<String> for In<BTreeSet<&str>> {
    fn matcher_type(&self) -> crate::MatchType {
        ToBe
    }
}
