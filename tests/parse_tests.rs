use http_message::PartialHttpRequest;
use http_message::error::Error;
use http_message::models::{request::HttpRequest, uri::Uri};

use pretty_assertions::assert_eq;

#[test]
fn parse_empty_request() {
    let content = include_str!("../tests/fixtures/empty.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::parsed(&content, None, None, None, vec![], None),
        partial
    );
}

#[test]
fn parse_whitespace_request() {
    let content = include_str!("../tests/fixtures/whitespace.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::parsed(&content, None, None, None, vec![], None),
        partial
    );
}

#[test]
fn parse_get_request() {
    let content = include_str!("../tests/fixtures/get.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::parsed(content, Some(0..3), Some(4..23), Some(24..32), vec![], None),
        partial
    );

    let request: Result<HttpRequest, Error> = partial.try_into();

    assert_eq!(
        Ok(HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: "HTTP/1.1".into(),
            headers: vec![],
            body: None
        }),
        request
    );
}

#[test]
fn parse_get_without_http_version_request() {
    let content = include_str!("../tests/fixtures/get_without_http_version.request");

    let partial = PartialHttpRequest::from_str(&content);

    assert_eq!(
        Ok(PartialHttpRequest::parsed(
            content,
            Some(0..3),
            Some(4..23),
            None,
            vec![],
            None
        )),
        partial
    );

    let request: Result<HttpRequest, Error> = partial.unwrap().try_into();

    assert_eq!(Err(Error::missing_required("http_version")), request);
}

#[test]
fn parse_get_with_headers_request() {
    let content = include_str!("../tests/fixtures/get_with_headers.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::parsed(
            content,
            Some(0..3),
            Some(4..23),
            Some(24..32),
            vec![33..51],
            None
        ),
        partial
    );

    let request: Result<HttpRequest, Error> = partial.try_into();

    assert_eq!(
        Ok(HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: "HTTP/1.1".into(),
            headers: vec!["x-api-key: abc123".into()],
            body: None
        }),
        request
    );
}

#[test]
fn parse_post_with_headers_and_body_request() {
    let content = include_str!("../tests/fixtures/post_with_headers_and_body.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    let method = Some(0..4);
    let uri = Some(5..24);
    let http_version = Some(25..33);
    let headers = vec![34..52];
    let body = Some(53..64);

    assert_eq!(
        PartialHttpRequest::parsed(&content, method, uri, http_version, headers, body),
        partial
    );

    let request: Result<HttpRequest, Error> = partial.try_into();

    assert_eq!(
        Ok(HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "POST".into(),
            http_version: "HTTP/1.1".into(),
            headers: vec!["x-api-key: abc123".into()],
            body: Some(String::from(r#"{"id": 100}"#))
        }),
        request
    );
}

#[test]
fn parse_post_with_body_request() {
    let content = include_str!("../tests/fixtures/post_with_body.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::parsed(
            content,
            Some(0..4),
            Some(5..24),
            Some(25..33),
            vec![],
            Some(35..46)
        ),
        partial
    );

    let request: Result<HttpRequest, Error> = partial.try_into();

    assert_eq!(
        Ok(HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "POST".into(),
            http_version: "HTTP/1.1".into(),
            headers: vec![],
            body: Some(String::from(r#"{"id": 100}"#))
        }),
        request
    );
}

#[test]
fn parse_get_with_multiple_spaces_request() {
    let content = include_str!("../tests/fixtures/get_with_multiple_spaces.request");

    let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

    assert_eq!(
        PartialHttpRequest::parsed(
            &content,
            Some(0..3),
            Some(5..24),
            Some(26..34),
            vec![],
            None
        ),
        partial
    );

    let request: Result<HttpRequest, Error> = partial.try_into();

    assert_eq!(
        Ok(HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: "HTTP/1.1".into(),
            headers: vec![],
            body: None
        }),
        request
    );
}
