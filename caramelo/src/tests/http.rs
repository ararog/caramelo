use crate::{
    expect,
    matchers::{self, path},
    Matcher,
};

#[test]
fn test_path_matcher() {
    let request = http::Request::get("/api/users")
        .body(())
        .unwrap();
    expect(request).to_have(path(r"^/api/.*$"));
}

#[test]
#[should_panic = "Expected Request { method: GET, uri: /users, version: HTTP/1.1, headers: {}, body: () } to have path matching Regex(\"^/api/.*$\")"]
fn test_path_matcher_panic() {
    let request = http::Request::get("/users")
        .body(())
        .unwrap();
    expect(request).to_have(path(r"^/api/.*$"));
}

#[test]
fn test_method_matcher() {
    let request = http::Request::get("/api/users")
        .body(())
        .unwrap();
    expect(request).to_have(matchers::method(http::Method::GET));
}

#[test]
#[should_panic = "Expected Request { method: POST, uri: /api/users, version: HTTP/1.1, headers: {}, body: () } to have method matching GET"]
fn test_method_matcher_panic() {
    let request = http::Request::post("/api/users")
        .body(())
        .unwrap();
    expect(request).to_have(matchers::method(http::Method::GET));
}
