use crate::Matcher;

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
///
/// # Examples
///
/// ```
/// use caramelo::expect;
/// use caramelo::matchers::contains;
///
/// expect("hello").to(contains("ell"));
/// ```
pub struct Contains(regex::Regex);

impl Matcher<String> for Contains {
    fn matches(&self, value: &String) -> bool {
        self.0
            .is_match(value)
    }

    fn matcher_type(&self) -> crate::MatchType {
        crate::MatchType::To
    }

    fn description(&self) -> String {
        format!("contains {:?}", self.0)
    }
}

impl Matcher<&str> for Contains {
    fn matches(&self, value: &&str) -> bool {
        self.0
            .is_match(value)
    }

    fn matcher_type(&self) -> crate::MatchType {
        crate::MatchType::To
    }

    fn description(&self) -> String {
        format!("contains {:?}", self.0)
    }
}
