use http_message::models::{HttpRequest, ParsedHttpRequest, Uri};

fn main() {
    let parsed =
        ParsedHttpRequest::from_str("GET https://example.com HTTP/1.1\nx-key: 123\n\n").unwrap();

    assert_eq!(&(0..3), parsed.method_span());
    assert_eq!("GET", parsed.method_str());

    assert_eq!(&(4..23), parsed.uri_span());
    assert_eq!("https://example.com", parsed.uri_str());

    assert_eq!(&(24..32), parsed.http_version_span());
    assert_eq!("HTTP/1.1", parsed.http_version_str());

    assert_eq!(Some(&(33..44)), parsed.header_span("x-key"));
    assert_eq!(Some("x-key: 123\n"), parsed.header_str("x-key"));

    let request: HttpRequest = parsed.into();

    assert_eq!(
        HttpRequest {
            uri: Uri::new("https://example.com"),
            method: "GET".into(),
            http_version: "HTTP/1.1".into(),
            headers: vec![("x-key", "123").into()],
            body: None
        },
        request
    );
}
