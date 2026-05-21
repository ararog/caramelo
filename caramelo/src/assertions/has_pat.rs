use std::fmt::Display;

/// Trait for regular expression matching assertions
pub trait HasPat: Display {
    /// Asserts that the value matches the regular expression
    ///
    /// # Arguments
    /// * `other` - The regular expression to match against
    ///
    /// # Panics
    /// * If the regular expression is invalid
    /// * If the value does not match the regular expression
    fn has_pat(&self, other: &str) {
        let regex = regex::Regex::new(other);
        if let Ok(regex) = regex {
            if !regex.is_match(&self.to_string()) {
                panic!("Expected \"{}\" to match \"{}\"", self, other);
            }
        } else {
            panic!("Invalid regex pattern: {}", other);
        }
    }
}

impl HasPat for &str {}
impl HasPat for String {}
