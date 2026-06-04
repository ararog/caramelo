use http::HeaderName;

use crate::Matcher;

/// Creates a matcher that checks if the request has the given header.
pub fn has_header<H>(value: H) -> HasHeader
where
    H: Into<HeaderName>,
{
    HasHeader(value.into())
}

/// A matcher that checks if the request has the given header.
pub struct HasHeader(http::header::HeaderName);

impl<T> Matcher<http::Request<T>> for HasHeader {
    fn matches(&self, value: &http::Request<T>) -> bool {
        value
            .headers()
            .contains_key(&self.0)
    }

    fn description(&self) -> String {
        format!("header matching {}", self.0)
    }
}

/// Creates a matcher that checks if the request path matches the given regex pattern.
pub fn has_header_value<N>(name: N, value: &str) -> HasHeaderName
where
    N: Into<HeaderName>,
{
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => HasHeaderName { name: name.into(), regex },
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// A matcher that checks if the request path matches a regex pattern.
pub struct HasHeaderName {
    name: HeaderName,
    regex: regex::Regex,
}

impl<T> Matcher<http::Request<T>> for HasHeaderName {
    fn matches(&self, value: &http::Request<T>) -> bool {
        value
            .headers()
            .get(&self.name)
            .is_some_and(|v| {
                self.regex.is_match(
                    v.to_str()
                        .unwrap_or(""),
                )
            })
    }

    fn description(&self) -> String {
        format!("header {} with value matching {:?}", self.name, self.regex)
    }
}
