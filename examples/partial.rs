use http_message::{
    error::Error,
    models::{HttpRequest, PartialHttpRequest},
    parse_partial_request,
};

fn main() {
    let partial: PartialHttpRequest<'_> =
        parse_partial_request("GET https://example.com\nx-key: 123").unwrap();

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
