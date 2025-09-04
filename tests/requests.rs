use http_message::models::request::HttpRequest;

#[test]
fn test() {
    HttpRequest::get("https://example.com", vec![("X-test", "").into()]);
}
