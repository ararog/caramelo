use crate::{
    and, expect,
    http::Request,
    matchers::{body, header, header_value, method, path},
    MatcherExt,
};
use http::{header::CONTENT_TYPE, Method, Uri};

#[test]
fn test_path_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(path(r"^/api/.*$"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /users, version: HTTP/1.1, headers: {}, body: None } to have path matching Regex(\"^/api/.*$\")"]
fn test_path_matcher_panic() {
    let request = Request::get(Uri::from_static("/users"))
        .empty()
        .unwrap();

    expect(request).to_have(and!(path(r"^/api/.*$"), method(Method::GET)));
}

#[test]
fn test_method_get_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::GET));
}

#[test]
fn test_method_post_matcher() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::POST));
}

#[test]
fn test_method_put_matcher() {
    let request = Request::put(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::PUT));
}

#[test]
fn test_method_delete_matcher() {
    let request = Request::delete(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::DELETE));
}

#[test]
fn test_method_patch_matcher() {
    let request = Request::patch(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::PATCH));
}

#[test]
fn test_method_options_matcher() {
    let request = Request::options(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::OPTIONS));
}

#[test]
fn test_method_trace_matcher() {
    let request = Request::trace(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::TRACE));
}

#[test]
fn test_method_connect_matcher() {
    let request = Request::connect(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::CONNECT));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: None } to have method matching GET"]
fn test_method_matcher_panic() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method("GET"));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: None } to have method matching POST and path matching Regex(\"^/api/posts$\")"]
fn test_method_and_path_matcher_panic() {
    let request = Request::post(Uri::from_static("/api/users"))
        .empty()
        .unwrap();

    expect(request).to_have(method(Method::POST).and(path(r"^/api/posts$")));
}

#[test]
fn test_header_matcher() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header(CONTENT_TYPE));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-store\": \"application/json\"}, body: None } to have header matching content-type"]
fn test_header_matcher_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-store", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header(CONTENT_TYPE));
}

#[test]
fn test_header_value_regex() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header_value("content-type", r"^application/.*"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /api/users, version: HTTP/1.1, headers: {\"content-type\": \"application/json\"}, body: None } to have header content-type with value matching Regex(\"^text/.*\")"]
fn test_header_value_regex_failure() {
    let request = Request::get(Uri::from_static("/api/users"))
        .header("content-type", "application/json")
        .empty()
        .unwrap();

    expect(request).to_have(header_value("content-type", r"^text/.*"));
}

#[test]
fn test_body_matcher() {
    let request = Request::post(Uri::from_static("/api/users"))
        .body("Hello, world!")
        .unwrap();

    expect(request).to_have(body("Hello, world!"));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: Some(\"Sometime soon!\") } to have body contents matching Regex(\"Hello, world!\")"]
fn test_body_matcher_failure() {
    let request = Request::post(Uri::from_static("/api/users"))
        .body("Sometime soon!")
        .unwrap();

    expect(request).to_have(body("Hello, world!"));
}

mod json_tests {
    use crate::{
        expect,
        http::Request,
        matchers::{exact_json_body, partial_json_body},
    };
    use http::Uri;
    use serde::Serialize;

    #[derive(Serialize)]
    struct User {
        name: String,
        age: u32,
    }

    #[test]
    fn test_exact_json_body_matcher() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"{"name":"John","age":30}"#)
            .unwrap();

        expect(request).to_have(exact_json_body(&User { name: "John".to_string(), age: 30 }));
    }

    #[test]
    #[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: Some(\"{\\\"name\\\":\\\"John\\\",\\\"age\\\":25}\") } to have body contents matching {\"name\":\"John\",\"age\":30}"]
    fn test_exact_json_body_matcher_failure() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"{"name":"John","age":25}"#)
            .unwrap();

        expect(request).to_have(exact_json_body(&User { name: "John".to_string(), age: 30 }));
    }

    #[test]
    fn test_partial_json_body_matcher() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"{"name":"John","age":30}"#)
            .unwrap();

        expect(request).to_have(partial_json_body("$.name"));
    }

    #[test]
    #[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: Some(\"{\\\"name\\\":\\\"John\\\",\\\"age\\\":25}\") } to have body contents matching $.genre"]
    fn test_patial_json_body_matcher_failure() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"{"name":"John","age":25}"#)
            .unwrap();

        expect(request).to_have(partial_json_body("$.genre"));
    }
}

mod xml_tests {
    use crate::{
        expect,
        http::Request,
        matchers::{exact_xml_body, partial_xml_body},
    };
    use http::Uri;
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename = "user")]
    struct User {
        name: String,
        age: u32,
    }

    #[test]
    fn test_exact_xml_body_matcher() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"<?xml version="1.0" encoding="UTF-8"?><user><name>John</name><age>30</age></user>"#)
            .unwrap();

        expect(request).to_have(exact_xml_body(&User { name: "John".to_string(), age: 30 }));
    }

    #[test]
    #[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: Some(\"<?xml version=\\\"1.0\\\" encoding=\\\"UTF-8\\\"?><user><name>John</name><age>25</age></user>\") } to have body contents matching <?xml version=\"1.0\" encoding=\"UTF-8\"?><user><name>John</name><age>30</age></user>"]
    fn test_exact_xml_body_matcher_failure() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"<?xml version="1.0" encoding="UTF-8"?><user><name>John</name><age>25</age></user>"#)
            .unwrap();

        expect(request).to_have(exact_xml_body(&User { name: "John".to_string(), age: 30 }));
    }

    #[test]
    fn test_partial_xml_body_matcher() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"<?xml version="1.0" encoding="UTF-8"?><user><name>John</name><age>30</age></user>"#)
            .unwrap();

        expect(request).to_have(partial_xml_body("//name"));
    }

    #[test]
    #[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: Some(\"<?xml version=\\\"1.0\\\" encoding=\\\"UTF-8\\\"?><user><name>John</name><age>25</age></user>\") } to have body contents matching //genre"]
    fn test_patial_xml_body_matcher_failure() {
        let request = Request::post(Uri::from_static("/api/users"))
            .body(r#"<?xml version="1.0" encoding="UTF-8"?><user><name>John</name><age>25</age></user>"#)
            .unwrap();

        expect(request).to_have(partial_xml_body("//genre"));
    }
}
