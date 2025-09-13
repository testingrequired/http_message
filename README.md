# http_message

```rust
use std::str::FromStr;

use http_message::PartialHttpRequest;

fn main() {
    let partial = PartialHttpRequest::from_str("GET https://example.com\nx-key: 123").unwrap();

    assert_eq!(Some(0..3), partial.method_span());
    assert_eq!(Some("GET"), partial.method_str());

    assert_eq!(Some(4..23), partial.uri_span());
    assert_eq!(Some("https://example.com"), partial.uri_str());

    assert_eq!(None, partial.http_version_span());
    assert_eq!(None, partial.http_version_str());

    assert_eq!(Some(24..34), partial.header_span("x-key"));
    assert_eq!(Some("x-key: 123"), partial.header_str("x-key"));
}
```

[![Verify](https://github.com/testingrequired/http_message/actions/workflows/verify.yml/badge.svg)](https://github.com/testingrequired/http_message/actions/workflows/verify.yml)
