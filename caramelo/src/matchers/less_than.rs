use crate::Matcher;
use std::fmt::Debug;

pub fn lt<T>(value: T) -> LessThan<T> {
    LessThan(value)
}

pub struct LessThan<T>(T);

impl<T> Matcher<T> for LessThan<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0 < *value
    }

    fn description(&self) -> String {
        format!("less than {:?}", self.0)
    }
}
