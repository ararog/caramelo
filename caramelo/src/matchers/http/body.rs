use crate::MatchType::ToHave;
use crate::{http::Request, Matcher, TypedMatcher};

pub use self::json::*;
pub use self::xml::*;

/// Creates a matcher that checks if the request body matches the given regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Body` - A matcher that checks if the request body matches the given regex pattern.
///
/// # Panics
///
/// * Panics if the regex pattern is invalid.
///
/// # Examples
///
/// ```rust
/// use caramelo::matchers::body;
///
/// let matcher = body(r"^Hello World$");
/// ```
pub fn body(value: &str) -> Body {
    let regex = regex::Regex::new(value);
    match regex {
        Ok(regex) => Body(regex),
        Err(_) => panic!("Invalid regex pattern"),
    }
}

/// A matcher that checks if the request body matches a regex pattern.
///
/// # Arguments
///
/// * `value` - The regex pattern to match against.
///
/// # Returns
///
/// * `Body` - A matcher that checks if the request body matches the given regex pattern.
///
/// # Examples
///
/// ```rust
/// use caramelo::matchers::body;
///
/// let matcher = body(r"^Hello World$");
/// ```
pub struct Body(regex::Regex);

impl Matcher<Request> for Body {
    fn matches(&self, value: &Request) -> bool {
        if let Some(body) = &value.body() {
            self.0
                .is_match(body)
        } else {
            false
        }
    }

    fn description(&self) -> String {
        format!("body contents matching {:?}", self.0)
    }
}

impl TypedMatcher<Request> for Body {
    fn matcher_type(&self) -> crate::MatchType {
        ToHave
    }
}

#[cfg(feature = "json")]
pub(crate) mod json {
    use jsonpath_rust::JsonPath;
    use sonic_rs::Serialize;

    use crate::{http::Request, MatchType::ToHave, Matcher, TypedMatcher};

    /// Creates a matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactJson` - A matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::exact_json_body;
    ///
    /// let matcher = exact_json_body(&serde_json::json!({"name": "John", "age": 30}));
    /// ```
    pub fn exact_json_body<T: Serialize>(value: &T) -> BodyWithExactJson {
        match sonic_rs::to_string(value) {
            Ok(json) => BodyWithExactJson(json),
            Err(e) => panic!("Failed to serialize JSON: {}", e),
        }
    }

    /// A matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactJson` - A matcher that checks if the request body matches the given JSON exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::exact_json_body;
    ///
    /// let matcher = exact_json_body(&serde_json::json!({"name": "John", "age": 30}));
    /// ```
    pub struct BodyWithExactJson(String);

    impl Matcher<Request> for BodyWithExactJson {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = value.body() {
                body == &self.0
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents matching {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithExactJson {
        fn matcher_type(&self) -> crate::MatchType {
            ToHave
        }
    }

    /// Creates a matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialJson` - A matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::partial_json_body;
    ///
    /// let matcher = partial_json_body(r#"$.name"#);
    /// ```
    pub fn partial_json_body(value: &str) -> BodyWithPartialJson {
        BodyWithPartialJson(value.to_owned())
    }

    /// A matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialJson` - A matcher that checks if the request body contains the given JSON partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::partial_json_body;
    ///
    /// let matcher = partial_json_body(r#"$.name"#);
    /// ```
    pub struct BodyWithPartialJson(String);

    impl Matcher<Request> for BodyWithPartialJson {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = &value.body() {
                if let Ok(json) = sonic_rs::from_str::<serde_json::Value>(body) {
                    if let Ok(results) = json.query_with_path(self.0.as_str()) {
                        !results.is_empty()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents containing {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithPartialJson {
        fn matcher_type(&self) -> crate::MatchType {
            ToHave
        }
    }
}

#[cfg(feature = "xml")]
pub(crate) mod xml {
    use serde::Serialize;
    use serde_xml_rs::to_string;
    use simdxml::parse;

    use crate::{http::Request, MatchType::ToHave, Matcher, TypedMatcher};

    /// Creates a matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactXml` - A matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::exact_xml_body;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let matcher = exact_xml_body(&User {
    ///     name: "John".to_string(),
    ///     age: 30,
    /// });
    /// ```
    pub fn exact_xml_body<T: Serialize>(value: &T) -> BodyWithExactXml {
        match to_string(value) {
            Ok(xml) => BodyWithExactXml(xml),
            Err(e) => panic!("Failed to serialize XML: {}", e),
        }
    }

    /// A matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML value to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithExactXml` - A matcher that checks if the request body matches the given XML exactly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::exact_xml_body;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let matcher = exact_xml_body(&User {
    ///     name: "John".to_string(),
    ///     age: 30,
    /// });
    /// ```
    pub struct BodyWithExactXml(String);

    impl Matcher<Request> for BodyWithExactXml {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = value.body() {
                body == &self.0
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents matching {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithExactXml {
        fn matcher_type(&self) -> crate::MatchType {
            ToHave
        }
    }

    /// Creates a matcher that checks if the request body contains the given XML partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialXml` - A matcher that checks if the request body contains the given XML partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::partial_xml_body;
    ///
    /// let matcher = partial_xml_body(r#"//name"#);
    /// ```
    pub fn partial_xml_body(value: &str) -> BodyWithPartialXml {
        BodyWithPartialXml(value.to_owned())
    }

    /// A matcher that checks if the request body contains the given XML partial.
    ///
    /// # Arguments
    ///
    /// * `value` - The XML partial to match against.
    ///
    /// # Returns
    ///
    /// * `BodyWithPartialXml` - A matcher that checks if the request body contains the given XML partial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::matchers::partial_xml_body;
    ///
    /// let matcher = partial_xml_body(r#"//name"#);
    /// ```
    pub struct BodyWithPartialXml(String);

    impl Matcher<Request> for BodyWithPartialXml {
        fn matches(&self, value: &Request) -> bool {
            if let Some(body) = &value.body() {
                if let Ok(xml) = parse(body.as_bytes()) {
                    if let Ok(results) = xml.xpath_string(self.0.as_str()) {
                        !results.is_empty()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn description(&self) -> String {
            format!("body contents containing {}", self.0)
        }
    }

    impl TypedMatcher<Request> for BodyWithPartialXml {
        fn matcher_type(&self) -> crate::MatchType {
            ToHave
        }
    }
}
