use std::fmt::Debug;

use crate::Matcher;

/// Creates a matcher that matches empty values
pub fn empty() -> Empty {
    Empty
}

/// Matcher that matches empty values
pub struct Empty;

impl<T> Matcher<T> for Empty
where
    T: PartialEq + Default + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value == T::default()
    }

    fn description(&self) -> String {
        "empty".to_owned()
    }
}
