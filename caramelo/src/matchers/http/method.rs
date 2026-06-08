use crate::Matcher;

pub trait AsMethod {
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
pub fn method<M>(value: M) -> Method
where
    M: AsMethod,
{
    Method(value.into_method())
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
