# http_message

Parse partial/non spec compliant HTTP messages.

## Partial HTTP Message

A `PartialHttpRequest` contains the potential spans for `method`, `uri`, `http_version`, `headers`, and `body`. The HTTP message does not need to be spec compliant so things like `http_version` are optional.

```rust
use http_message::{
    error::Error,
    models::{HttpRequest, PartialHttpRequest},
};

fn main() {
    let partial = PartialHttpRequest::from_str("GET https://example.com\nx-key: 123").unwrap();

    assert_eq!(&Some(0..3), partial.method_span());
    assert_eq!(Some("GET"), partial.method_str());

    assert_eq!(&Some(4..23), partial.uri_span());
    assert_eq!(Some("https://example.com"), partial.uri_str());

    assert_eq!(&None, partial.http_version_span());
    assert_eq!(None, partial.http_version_str());

    assert_eq!(Some(&(24..34)), partial.header_span("x-key"));
    assert_eq!(Some("x-key: 123"), partial.header_str("x-key"));

    let request: Result<HttpRequest, Error> = partial.try_into();

    assert_eq!(Err(Error::missing_required("http_version")), request);
}

```

## Parsed HTTP Message

A `ParsedHttpRequest` contains the spans for `method`, `uri`, `http_version`, `headers`, and `body`. The HTTP message does need to be spec compliant so things like `http_version` are required.

```rust
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
```

[![Verify](https://github.com/testingrequired/http_message/actions/workflows/verify.yml/badge.svg)](https://github.com/testingrequired/http_message/actions/workflows/verify.yml)
