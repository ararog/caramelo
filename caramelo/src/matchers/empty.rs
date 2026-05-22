use std::fmt::Debug;

use crate::Matcher;

/// Creates a matcher that matches empty values
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::empty;
///
/// expect("").to_be(empty());
/// ```
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
