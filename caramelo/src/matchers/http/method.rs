use crate::{http::Request, MatchType::ToHave, Matcher, TypedMatcher};

/// Trait for converting values into http::Method.
pub trait AsMethod {
    /// Converts the value into a http::Method.
    fn into_method(self) -> http::Method;
}

impl AsMethod for http::Method {
    fn into_method(self) -> http::Method {
        self
    }
}

impl AsMethod for String {
    fn into_method(self) -> http::Method {
        self.as_str()
            .into_method()
    }
}

impl AsMethod for &str {
    fn into_method(self) -> http::Method {
        match self {
            "GET" | "get" => http::Method::GET,
            "POST" | "post" => http::Method::POST,
            "PUT" | "put" => http::Method::PUT,
            "DELETE" | "delete" => http::Method::DELETE,
            "PATCH" | "patch" => http::Method::PATCH,
            "HEAD" | "head" => http::Method::HEAD,
            "OPTIONS" | "options" => http::Method::OPTIONS,
            _ => panic!("Invalid method"),
        }
    }
}

/// Creates a matcher that checks if the request method matches the given method.
///
/// # Arguments
///
/// * `value` - The method to match against.
///
/// # Returns
///
/// * `Method` - A matcher that checks if the request method matches the given method.
pub fn method<M>(value: M) -> Method
where
    M: AsMethod,
{
    Method(value.into_method())
}

/// A matcher that checks if the request method matches a specific HTTP method.
///
/// # Arguments
///
/// * `method` - The HTTP method to match against.
///
/// # Returns
///
/// * `Method` - A matcher that checks if the request method matches the given method.
///
/// # Examples
///
/// ```rust
/// use caramelo::matchers::method;
///
/// let matcher = method("GET");
/// ```
pub struct Method(http::Method);

impl Matcher<Request> for Method {
    fn matches(&self, value: &Request) -> bool {
        self.0 == value.method()
    }

    fn description(&self) -> String {
        format!("method matching {}", self.0)
    }
}

impl TypedMatcher<Request> for Method {
    fn matcher_type(&self) -> crate::MatchType {
        ToHave
    }
}
