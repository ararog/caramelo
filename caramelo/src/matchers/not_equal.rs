use crate::Matcher;
use std::fmt::Debug;

pub fn ne<T>(value: T) -> NotEqual<T> {
    NotEqual(value)
}

pub struct NotEqual<T>(T);

impl<T> Matcher<T> for NotEqual<T>
where
    T: PartialEq + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 != *value
    }

    fn description(&self) -> String {
        format!("not equal to {:?}", self.0)
    }
}
