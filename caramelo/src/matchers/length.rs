use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use crate::{
    MatchType::{self, ToHave},
    Matcher, TypedMatch,
};

/// Creates a matcher that matches values with the given length
///
/// # Examples
///
/// ```
/// use caramelo::{expect, matchers::length};
///
/// expect(vec![1, 2, 3]).to_have(length(3));
/// ```
pub fn length(value: usize) -> Length {
    Length(value)
}

/// Creates a matcher that matches values with the given length (alias for length)
///
/// # Examples
///
/// ```
/// use caramelo::{expect, matchers::len};
///
/// expect(vec![1, 2, 3]).to_have(len(3));
/// ```
pub fn len(value: usize) -> Length {
    length(value)
}

/// Matcher that matches values with a specific length
///
/// # Examples
///
/// ```
/// use caramelo::{expect, matchers::length};
///
/// expect(vec![1, 2, 3]).to_have(length(3));
/// ```
pub struct Length(usize);

impl<T> Matcher<Vec<T>> for Length {
    fn matches(&self, value: &Vec<T>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<T> TypedMatch<Vec<T>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}

impl<T> Matcher<VecDeque<T>> for Length {
    fn matches(&self, value: &VecDeque<T>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<T> TypedMatch<VecDeque<T>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}

impl<T> Matcher<LinkedList<T>> for Length {
    fn matches(&self, value: &LinkedList<T>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<T> TypedMatch<LinkedList<T>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}

impl<K, V> Matcher<HashMap<K, V>> for Length {
    fn matches(&self, value: &HashMap<K, V>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<K, V> TypedMatch<HashMap<K, V>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}

impl<K, V> Matcher<BTreeMap<K, V>> for Length {
    fn matches(&self, value: &BTreeMap<K, V>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<K, V> TypedMatch<BTreeMap<K, V>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}

impl<K, V> Matcher<HashSet<K, V>> for Length {
    fn matches(&self, value: &HashSet<K, V>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<K, V> TypedMatch<HashSet<K, V>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}

impl<T> Matcher<BTreeSet<T>> for Length {
    fn matches(&self, value: &BTreeSet<T>) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}

impl<T> TypedMatch<BTreeSet<T>> for Length {
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}
