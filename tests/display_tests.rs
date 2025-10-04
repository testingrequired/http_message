use std::fs;

use http_message::PartialHttpRequest;

use pretty_assertions::assert_eq;

test!(display_empty_request, "./tests/fixtures/empty.request");
test!(
    display_get_with_headers_request,
    "./tests/fixtures/get_with_headers.request"
);
test!(
    display_get_with_multiple_spaces_request,
    "./tests/fixtures/get_with_multiple_spaces.request"
);
test!(
    display_get_without_http_version_request,
    "./tests/fixtures/get_without_http_version.request"
);
test!(display_get_request, "./tests/fixtures/get.request");
test!(
    display_post_with_body_request,
    "./tests/fixtures/post_with_body.request"
);
test!(
    display_post_with_headers_and_body_request,
    "./tests/fixtures/post_with_headers_and_body.request"
);
test!(
    display_whitespace_request,
    "./tests/fixtures/whitespace.request"
);

#[macro_export]
macro_rules! test {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            let path: &str = $path;
            let content = fs::read_to_string(path).expect("should read test fixture");

            let partial = PartialHttpRequest::from_str(&content).expect("should be parsable");

            assert_eq!(content, format!("{partial}"));
        }
    };
}
