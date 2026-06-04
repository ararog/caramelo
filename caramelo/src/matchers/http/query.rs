use crate::Matcher;

/// Creates a matcher that checks if the request query matches the given regex pattern.
pub fn query(value: &str) -> Query {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Query(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// A matcher that checks if the request query matches a regex pattern.
pub struct Query(regex::Regex);

impl<T> Matcher<http::Request<T>> for Query {
    fn matches(&self, value: &http::Request<T>) -> bool {
        self.0.is_match(
            value
                .uri()
                .query()
                .unwrap_or(""),
        )
    }

    fn description(&self) -> String {
        format!("query matching {:?}", self.0)
    }
}
