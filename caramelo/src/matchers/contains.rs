use crate::Matcher;
use std::fmt::Debug;

/// Creates a matcher that matches values containing the given string
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::contains;
///
/// expect("hello").to(contains("ell"));
/// ```
pub fn contains(value: &str) -> Contains {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Contains(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// Matcher that matches values equal to the given value
pub struct Contains(regex::Regex);

impl<T> Matcher<T> for Contains
where
    T: ToString + Debug,
{
    fn matches(&self, value: &T) -> bool {
        self.0
            .is_match(&value.to_string())
    }

    fn description(&self) -> String {
        format!("contains {:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;

    use super::*;

    #[test]
    fn test_contains() {
        let regex = regex::Regex::new("ell").unwrap();
        expect("hello").to(Contains(regex));
    }
}
