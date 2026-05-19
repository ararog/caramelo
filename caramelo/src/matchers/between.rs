use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values between min and max (inclusive)
pub fn between<T>(min: T, max: T) -> Between<T> {
    Between { min, max }
}

/// Matcher that matches values between min and max (inclusive)
pub struct Between<T> {
    min: T,
    max: T,
}

impl<T> Matcher<T> for Between<T>
where
    T: PartialOrd + Debug,
{
    fn matches(&self, value: &T) -> bool {
        value >= &self.min && value <= &self.max
    }

    fn description(&self) -> String {
        format!("between {:?} and {:?}", self.min, self.max)
    }
}
