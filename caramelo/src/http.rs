use http::Uri;
use std::ops::Deref;

/// A builder for creating http requests
#[derive(Debug)]
pub struct RequestBuilder {
    method: http::Method,
    uri: Uri,
    version: http::Version,
    headers: http::HeaderMap,
}

impl RequestBuilder {
    /// Sets the request method
    pub fn method(self, method: http::Method) -> Self {
        Self { method, ..self }
    }

    /// Sets the request version
    pub fn version(self, version: http::Version) -> Self {
        Self { version, ..self }
    }

    /// Sets the request path
    pub fn uri(self, uri: Uri) -> Self {
        Self { uri, ..self }
    }

    /// Sets the request headers
    pub fn header<K>(self, name: K, value: &str) -> Self
    where
        K: http::header::IntoHeaderName,
    {
        let mut headers = self.headers;
        headers.insert(
            name,
            value
                .parse()
                .unwrap(),
        );
        Self { headers, ..self }
    }

    /// Creates an empty request
    pub fn empty(self) -> Result<Request, http::Error> {
        Ok(Request {
            method: self.method,
            version: self.version,
            uri: self.uri,
            headers: self.headers,
            body: None,
        })
    }

    /// Builds the request
    pub fn body(self, body: &str) -> Result<Request, http::Error> {
        Ok(Request {
            method: self.method,
            version: self.version,
            uri: self.uri,
            headers: self.headers,
            body: Some(body.to_string()),
        })
    }
}

/// A http request wrapper
#[derive(Debug, PartialEq)]
pub struct Request {
    method: http::Method,
    uri: Uri,
    version: http::Version,
    headers: http::HeaderMap,
    body: Option<String>,
}

impl Request {
    fn builder(method: http::Method, uri: Uri) -> RequestBuilder {
        RequestBuilder {
            method,
            version: http::Version::HTTP_11,
            uri,
            headers: http::HeaderMap::new(),
        }
    }

    /// Creates a new GET request builder
    pub fn get(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::GET, uri)
    }

    /// Creates a new POST request builder
    pub fn post(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::POST, uri)
    }

    /// Creates a new PUT request builder
    pub fn put(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::PUT, uri)
    }

    /// Creates a new DELETE request builder
    pub fn delete(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::DELETE, uri)
    }

    /// Creates a new PATCH request builder
    pub fn patch(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::PATCH, uri)
    }

    /// Creates a new HEAD request builder
    pub fn head(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::HEAD, uri)
    }

    /// Creates a new OPTIONS request builder
    pub fn options(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::OPTIONS, uri)
    }

    /// Creates a new TRACE request builder
    pub fn trace(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::TRACE, uri)
    }

    /// Creates a new CONNECT request builder
    pub fn connect(uri: Uri) -> RequestBuilder {
        Self::builder(http::Method::CONNECT, uri)
    }

    /// Creates a new request from http parts and body, the goal is to provide
    /// a http request wrapper that can be used in tests with body already
    /// parsed into a string.
    ///
    /// # Arguments
    ///
    /// * `parts` - The http request parts
    /// * `body` - The request body
    ///
    /// # Returns
    ///
    /// * `Self` - The new request
    ///
    /// # Panics
    ///
    /// * Panics if the uri is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use caramelo::http::Request;
    /// use http::{Method, Uri};
    ///
    /// let (parts, _) = http::request::Request::new(()).into_parts();
    ///
    /// let request = Request::from_parts(parts, "body".to_string());
    /// ```
    pub fn from_parts(parts: http::request::Parts, body: String) -> Self {
        Self {
            method: parts.method,
            version: parts.version,
            uri: parts.uri,
            headers: parts.headers,
            body: Some(body),
        }
    }

    /// Returns the request method
    pub fn method(&self) -> &http::Method {
        &self.method
    }

    /// Returns the request version
    pub fn version(&self) -> &http::Version {
        &self.version
    }

    /// Returns the request path
    pub fn path(&self) -> &str {
        self.uri.path()
    }

    /// Returns the request query string without the leading question mark
    pub fn query(&self) -> &str {
        self.uri
            .query()
            .unwrap_or("")
    }

    /// Returns the request headers
    pub fn headers(&self) -> &http::HeaderMap {
        &self.headers
    }

    /// Returns the request body
    pub fn body(&self) -> &Option<String> {
        &self.body
    }
}

impl Deref for Request {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}
