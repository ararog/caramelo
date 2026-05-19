use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values equal to the given value
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
