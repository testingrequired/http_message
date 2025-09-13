use std::{fs, str::FromStr};

use http_message::PartialHttpRequest;
use http_message::models::{request::HttpRequest, uri::Uri};

use pretty_assertions::assert_eq;

#[test]
fn parse_empty_request() {
    let content =
        fs::read_to_string("./tests/fixtures/empty.request").expect("should read test fixture");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::new(&content, None, None, None, vec![], None),
        partial
    );
}

#[test]
fn parse_whitespace_request() {
    let content = fs::read_to_string("./tests/fixtures/whitespace.request")
        .expect("should read test fixture");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::new(&content, None, None, None, vec![], None),
        partial
    );
}

#[test]
fn parse_get_request() {
    let content =
        fs::read_to_string("./tests/fixtures/get.request").expect("should read test fixture");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

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
            http_version: "HTTP/1.1".into(),
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

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

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
            http_version: "HTTP/1.1".into(),
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

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

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
            http_version: "HTTP/1.1".into(),
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

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

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
            http_version: "HTTP/1.1".into(),
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

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

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
            http_version: "HTTP/1.1".into(),
            headers: vec![],
            body: Some(String::from(r#"{"id": 100}"#))
        },
        request
    );
}

#[test]
fn parse_get_with_multiple_spaces_request() {
    let content = fs::read_to_string("./tests/fixtures/get_with_multiple_spaces.request")
        .expect("should read test fixture");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::new(
            &content,
            Some(5..24),
            Some(0..3),
            Some(26..34),
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
            http_version: "HTTP/1.1".into(),
            headers: vec![],
            body: None
        },
        request
    );
}
