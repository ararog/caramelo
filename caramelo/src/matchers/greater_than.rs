use crate::Matcher;
use std::fmt::Debug;

pub fn gt<T>(value: T) -> GreaterThan<T> {
    GreaterThan(value)
}

pub struct GreaterThan<T>(T);

impl<T> Matcher<T> for GreaterThan<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 > *value
    }

    fn description(&self) -> String {
        format!("greater than {:?}", self.0)
    }
}
