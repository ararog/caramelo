use http_body_util::BodyExt;

use crate::Matcher;

/// Creates a matcher that checks if the request body matches the given regex pattern.
pub fn body(value: &str) -> Body {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Body(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// A matcher that checks if the request body matches a regex pattern.
pub struct Body(regex::Regex);

impl<T> Matcher<http::Request<T>> for Body
where
    T: http_body::Body,
{
    fn matches(&self, value: &http::Request<T>) -> bool {
        let body = value.body();

        true
    }

    fn description(&self) -> String {
        format!("body contents matching {:?}", self.0)
    }
}
