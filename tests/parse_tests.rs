use std::fs;

use http_message::{
    models::{partial_request::PartialHttpRequest, request::HttpRequest, uri::Uri},
    parse::parse_request,
};

#[test]
fn parse_get_request() {
    let content =
        fs::read_to_string("./tests/fixtures/get.request").expect("should read test fixture");

    let partial: PartialHttpRequest = parse_request(&content);

    assert_eq!(
        PartialHttpRequest {
            uri: "https://example.com".to_string(),
            method: "GET".to_string(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec![],
            body: None
        },
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec![],
            body: None
        },
        request
    );
}

#[test]
fn parse_get_without_http_version_request() {
    let content = fs::read_to_string("./tests/fixtures/get_without_http_version.request")
        .expect("should read test fixture");

    let partial: PartialHttpRequest = parse_request(&content);

    assert_eq!(
        PartialHttpRequest {
            uri: "https://example.com".to_string(),
            method: "GET".to_string(),
            http_version: None,
            headers: vec![],
            body: None
        },
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec![],
            body: None
        },
        request
    );
}

#[test]
fn parse_get_with_headers_request() {
    let content = fs::read_to_string("./tests/fixtures/get_with_headers.request")
        .expect("should read test fixture");

    let partial = parse_request(&content);

    assert_eq!(
        PartialHttpRequest {
            uri: "https://example.com".to_string(),
            method: "GET".to_string(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec!["x-api-key: abc123".into()],
            body: None
        },
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec!["x-api-key: abc123".into()],
            body: None
        },
        request
    );
}

#[test]
fn parse_post_with_headers_and_body_request() {
    let content = fs::read_to_string("./tests/fixtures/post_with_headers_and_body.request")
        .expect("should read test fixture");

    let partial = parse_request(&content);

    assert_eq!(
        PartialHttpRequest {
            uri: "https://example.com".to_string(),
            method: "POST".to_string(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec!["x-api-key: abc123".into()],
            body: Some(String::from(r#"{"id": 100}"#))
        },
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "POST".into(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec!["x-api-key: abc123".into()],
            body: Some(String::from(r#"{"id": 100}"#))
        },
        request
    );
}

#[test]
fn parse_post_with_body_request() {
    let content = fs::read_to_string("./tests/fixtures/post_with_body.request")
        .expect("should read test fixture");

    let partial = parse_request(&content);

    assert_eq!(
        PartialHttpRequest {
            uri: "https://example.com".to_string(),
            method: "POST".to_string(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec![],
            body: Some(String::from(r#"{"id": 100}"#))
        },
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "POST".into(),
            http_version: Some("HTTP/1.1".into()),
            headers: vec![],
            body: Some(String::from(r#"{"id": 100}"#))
        },
        request
    );
}
