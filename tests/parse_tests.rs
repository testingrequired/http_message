use std::fs;

use http_message::{
    models::{partial_request::PartialHttpRequest, request::HttpRequest, uri::Uri},
    parse::parse_request,
};

use pretty_assertions::assert_eq;

#[test]
fn parse_get_request() {
    let content =
        fs::read_to_string("./tests/fixtures/get.request").expect("should read test fixture");

    let partial: PartialHttpRequest = parse_request(&content);

    assert_eq!(
        PartialHttpRequest::new(
            include_str!("../tests/fixtures/get.request"),
            Some(4..23),
            Some(0..3),
            Some(24..32),
            vec![],
            None
        ),
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
        PartialHttpRequest::new(
            include_str!("../tests/fixtures/get_without_http_version.request"),
            Some(4..23),
            Some(0..3),
            None,
            vec![],
            None
        ),
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
        PartialHttpRequest::new(
            include_str!("../tests/fixtures/get_with_headers.request"),
            Some(4..23),
            Some(0..3),
            Some(24..32),
            vec![33..51],
            None
        ),
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

    let method = Some(0..4);
    let uri = Some(5..24);
    let http_version = Some(25..33);
    let headers = vec![34..52];
    let body = Some(53..64);

    assert_eq!(
        PartialHttpRequest::new(&content, uri, method, http_version, headers, body),
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
        PartialHttpRequest::new(
            include_str!("../tests/fixtures/post_with_body.request"),
            Some(5..24),
            Some(0..4),
            Some(25..33),
            vec![],
            Some(35..46)
        ),
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
