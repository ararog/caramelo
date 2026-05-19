use crate::Matcher;
use std::fmt::{Debug, Display};

/// Creates a matcher that matches vectors containing the given item
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
