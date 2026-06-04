use crate::Matcher;

/// Creates a matcher that checks if the request path matches the given regex pattern.
pub fn path(value: &str) -> Path {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Path(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// A matcher that checks if the request path matches a regex pattern.
pub struct Path(regex::Regex);

impl<T> Matcher<http::Request<T>> for Path {
    fn matches(&self, value: &http::Request<T>) -> bool {
        self.0
            .is_match(value.uri().path())
    }

    fn description(&self) -> String {
        format!("path matching {:?}", self.0)
    }
}
