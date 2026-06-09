use crate::{http::Request, Matcher};

/// Creates a matcher that checks if the request path matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Path` - A matcher that checks if the request path matches the given regex pattern.
///
/// # Panics
///
/// * `Invalid regex pattern` - If the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use caramelo::matchers::path;
///
/// let matcher = path(r"^/api/v1/.*$");
/// ```
pub fn path(value: &str) -> Path {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Path(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// A matcher that checks if the request path matches a regex pattern.
///
/// # Arguments
///
/// * `regex` - The regex pattern to match against.
///
/// # Returns
///
/// * `Path` - A matcher that checks if the request path matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use caramelo::matchers::path;
///
/// let matcher = path(r"^/api/v1/.*$");
/// ```
pub struct Path(regex::Regex);

impl Matcher<Request> for Path {
    fn matches(&self, value: &Request) -> bool {
        self.0
            .is_match(value.path())
    }

    fn description(&self) -> String {
        format!("path matching {:?}", self.0)
    }
}
