use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values greater than the given value
pub fn gt<T>(value: T) -> GreaterThan<T> {
    GreaterThan(value)
}

/// Matcher that matches values greater than the given value
pub struct GreaterThan<T>(T);

impl<T> Matcher<T> for GreaterThan<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value > self.0
    }

    fn description(&self) -> String {
        format!("greater than {:?}", self.0)
    }
}

/// Creates a matcher that matches values greater than or equal to the given value
pub fn ge<T>(value: T) -> GreaterThanOrEqual<T> {
    GreaterThanOrEqual(value)
}

/// Matcher that matches values greater than or equal to the given value
pub struct GreaterThanOrEqual<T>(T);

impl<T> Matcher<T> for GreaterThanOrEqual<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        *value >= self.0
    }

    fn description(&self) -> String {
        format!("greater than or equal to {:?}", self.0)
    }
}
