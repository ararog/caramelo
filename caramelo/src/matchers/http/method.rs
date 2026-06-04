use crate::Matcher;

/// Creates a matcher that checks if the request method matches the given method.
pub fn method<M>(value: M) -> Method
where
    M: Into<http::Method>,
{
    Method(value.into())
}

/// A matcher that checks if the request method matches a specific HTTP method.
pub struct Method(http::Method);

impl<T> Matcher<http::Request<T>> for Method {
    fn matches(&self, value: &http::Request<T>) -> bool {
        self.0 == value.method()
    }

    fn description(&self) -> String {
        format!("method matching {}", self.0)
    }
}
