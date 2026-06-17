use crate::{
    MatchType::{self, ToHave},
    Matcher, TypedMatcher,
};
use std::fmt::{Debug, Display};

/// Creates a matcher that matches vectors containing the given item
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::item;
///
/// expect(vec![1, 2, 3]).to_have(item(2));
/// ```
pub fn item<I>(item: I) -> Item<I> {
    Item { item }
}

/// Matcher that matches vectors containing the given item
pub struct Item<T> {
    item: T,
}

impl<T> Matcher<Vec<T>> for Item<T>
where
    T: PartialEq + Display + Debug,
{
    fn matches(&self, value: &Vec<T>) -> bool {
        value.contains(&self.item)
    }

    fn description(&self) -> String {
        format!("item {}", self.item)
    }
}

impl<T> TypedMatcher<Vec<T>> for Item<T>
where
    T: PartialEq + Display + Debug,
{
    fn matcher_type(&self) -> MatchType {
        ToHave
    }
}
