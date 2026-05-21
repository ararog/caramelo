use std::fmt::Display;

/// Trait for equality assertions
pub trait Has: Display {
    /// Asserts that the value is equal to the expected value
    fn has(&self, other: &Self) {
        let self_str = self.to_string();
        let other_str = other.to_string();

        if !self_str.contains(&other_str) {
            panic!("Expected {} to contain {}", self, other);
        }
    }
}

impl Has for &str {}
impl Has for String {}
