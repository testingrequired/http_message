use std::fs;

use http_message::models::{partial_request::PartialHttpRequest, request::HttpRequest};

#[test]
fn parse_get_request() {
    // Read file ./fixtures/get_examplecom.request
    let content =
        fs::read_to_string("./tests/fixtures/get.request").expect("should read test fixture");

    let partial: PartialHttpRequest = http_message::parse(&content);

    assert_eq!(
        PartialHttpRequest::get("https://example.com", vec![]),
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(HttpRequest::get("https://example.com", vec![]), request);
}

#[test]
fn parse_get_with_headers_request() {
    // Read file ./fixtures/get_examplecom.request
    let content = fs::read_to_string("./tests/fixtures/get_with_headers.request")
        .expect("should read test fixture");

    let partial = http_message::parse(&content);

    assert_eq!(
        PartialHttpRequest::get("https://example.com", vec!["x-api-key: abc123".into()]),
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest::get("https://example.com", vec!["x-api-key: abc123".into()]),
        request
    );
}

#[test]
fn parse_post_with_headers_and_body_request() {
    // Read file ./fixtures/get_examplecom.request
    let content = fs::read_to_string("./tests/fixtures/post_with_headers_and_body.request")
        .expect("should read test fixture");

    let partial = http_message::parse(&content);

    assert_eq!(
        PartialHttpRequest::post(
            "https://example.com",
            vec!["x-api-key: abc123".into()],
            Some(String::from(r#"{"id": 100}"#))
        ),
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest::post(
            "https://example.com",
            vec!["x-api-key: abc123".into()],
            Some(String::from(r#"{"id": 100}"#))
        ),
        request
    );
}

#[test]
fn parse_post_with_body_request() {
    // Read file ./fixtures/get_examplecom.request
    let content = fs::read_to_string("./tests/fixtures/post_with_body.request")
        .expect("should read test fixture");

    let partial = http_message::parse(&content);

    assert_eq!(
        PartialHttpRequest::post(
            "https://example.com",
            vec![],
            Some(String::from(r#"{"id": 100}"#))
        ),
        partial
    );

    let request: HttpRequest = partial.into();

    assert_eq!(
        HttpRequest::post(
            "https://example.com",
            vec![],
            Some(String::from(r#"{"id": 100}"#))
        ),
        request
    );
}
