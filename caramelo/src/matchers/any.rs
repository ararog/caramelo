use crate::Matcher;
use std::{fmt::Debug, marker::PhantomData};

/// Creates a matcher that matches any value
pub fn any<T>() -> Any<T> {
    Any { _marker: PhantomData }
}

/// Matcher that matches any value
pub struct Any<T> {
    _marker: PhantomData<T>,
}

impl<T> Matcher<T> for Any<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, _value: &T) -> bool {
        true
    }

    fn description(&self) -> String {
        "any value".to_string()
    }
}
