use crate::Matcher;

/// Creates a matcher that matches values with the given length
pub fn length(value: usize) -> Length {
    Length(value)
}

/// Creates a matcher that matches values with the given length (alias for length)
pub fn len(value: usize) -> Length {
    length(value)
}

pub struct Length(usize);

impl<T> Matcher<T> for Length
where
    T: std::ops::Deref<Target = [u8]>,
{
    fn matches(&self, value: &T) -> bool {
        self.0 == value.len()
    }

    fn description(&self) -> String {
        format!("length equals to {}", self.0)
    }
}
