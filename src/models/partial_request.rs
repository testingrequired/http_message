use core::fmt;
use std::ops::Range;

use crate::{
    error::Error,
    span::{Span, get_line_spans},
};

/// A partial HTTP request that might not conform to HTTP spec
///
/// A templated HTTP request message is an example use case.
#[derive(Debug, PartialEq)]
pub struct PartialHttpRequest<'http_message> {
    message: &'http_message str,
    method: Option<Range<usize>>,
    uri: Option<Range<usize>>,
    http_version: Option<Range<usize>>,
    headers: Vec<Range<usize>>,
    body: Option<Range<usize>>,
}

impl<'http_message> fmt::Display for PartialHttpRequest<'http_message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<'http_message> PartialHttpRequest<'http_message> {
    pub fn from_str(message: &'http_message str) -> Result<Self, Error> {
        parse_request(message, parse_first_line)
    }

    pub fn parsed(
        message: &'http_message str,
        method: Option<Range<usize>>,
        uri: Option<Range<usize>>,
        http_version: Option<Range<usize>>,
        headers: Vec<Range<usize>>,
        body: Option<Range<usize>>,
    ) -> Self {
        let partial = Self {
            message,
            method,
            uri,
            http_version,
            headers,
            body,
        };

        partial.verify_spans();

        partial
    }

    /// Verify all the spans in the struct are valid
    ///
    /// - Aren't out of bounds of the message
    /// - Parts aren't overlapping or out of order
    fn verify_spans(&self) {
        self.method.as_ref().inspect(|span| {
            assert!(span.start < span.end);
            assert_text_span(self.message(), span);
        });

        self.uri.as_ref().inspect(|span| {
            assert!(span.start < span.end);
            assert_text_span(self.message(), span);

            if let Some(method) = self.method_span() {
                if !(method.start < span.start && method.end < span.start) {
                    panic!("uri {span:?} and method {method:?} spans conflict");
                }
            }
        });

        self.http_version.as_ref().inspect(|span| {
            assert!(span.start < span.end);
            assert_text_span(self.message(), span);

            if let Some(uri) = self.uri_span() {
                if !(uri.start < span.start && uri.end < span.start) {
                    panic!("http version {span:?} and uri {uri:?} spans conflict");
                }
            }
        });

        for span in self.header_spans().iter() {
            assert!(span.start < span.end);
            assert_text_span(self.message(), span);
        }

        self.body.as_ref().inspect(|span| {
            assert!(span.start < span.end);
            assert_text_span(self.message(), span);
        });
    }

    /// Get the original HTTP request message text
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the text span of the uri, if defined
    pub fn uri_span(&self) -> &Option<Range<usize>> {
        &self.uri
    }

    /// Get the string text of the uri, if defined
    pub fn uri_str(&self) -> Option<&str> {
        self.uri.as_ref().map(|span| self.slice_message(span))
    }

    /// Get the text span of the method, if defined
    pub fn method_span(&self) -> &Option<Range<usize>> {
        &self.method
    }

    /// Get the string text of the method, if defined
    pub fn method_str(&self) -> Option<&str> {
        self.method.as_ref().map(|span| self.slice_message(span))
    }

    /// Get the text span of the http version, if defined
    pub fn http_version_span(&self) -> &Option<Range<usize>> {
        &self.http_version
    }

    /// Get the string text of the http version, if defined
    pub fn http_version_str(&self) -> Option<&str> {
        self.http_version
            .as_ref()
            .map(|span| self.slice_message(span))
    }

    /// Get a list of the header line text spans
    pub fn header_spans(&self) -> &Vec<Range<usize>> {
        &self.headers
    }

    /// Get a list of the string text header lines
    pub fn header_strs(&self) -> Vec<&str> {
        self.headers
            .iter()
            .map(|span| self.slice_message(span))
            .collect()
    }

    /// Get the text span of a header line by key, if defined
    pub fn header_span(&self, key: &str) -> Option<&Range<usize>> {
        self.headers
            .iter()
            .find(|span| self.slice_message(span).starts_with(&format!("{key}:")))
    }

    /// Get the string text of a header by key, if defined
    pub fn header_str(&self, key: &str) -> Option<&str> {
        self.header_span(key).map(|span| self.slice_message(span))
    }

    /// Get the string text of the body, if defined
    pub fn body_str(&self) -> Option<&str> {
        self.body.as_ref().map(|span| &self.message[span.clone()])
    }

    /// Return a slice of the message string
    fn slice_message(&self, span: &Span) -> &str {
        &self.message[span.clone()]
    }
}

fn assert_text_span(text: &str, span: &Range<usize>) {
    text.get(span.clone())
        .expect(&format!("span {:?} is outside of text bounds", span));
}

impl<'http_message> Default for PartialHttpRequest<'http_message> {
    fn default() -> Self {
        Self::from_str("GET https://example.com HTTP/1.1").unwrap()
    }
}

type FirstLineParts = (
    Option<Range<usize>>,
    Option<Range<usize>>,
    Option<Range<usize>>,
);

fn parse_request<'http_message, F>(
    input: &'http_message str,
    parse_first_line: F,
) -> Result<PartialHttpRequest<'http_message>, Error>
where
    F: Fn(&str) -> FirstLineParts,
{
    if input.trim().is_empty() {
        return Ok(PartialHttpRequest::parsed(
            input,
            None,
            None,
            None,
            vec![],
            None,
        ));
    }

    let line_spans = get_line_spans(input);

    let first_empty_line_idx = line_spans.iter().position(|span| span.len() == 1);

    let first_line = line_spans.first();

    let (method, uri, http_version) = first_line
        .map(|span| &input[span.clone()])
        .map(parse_first_line)
        .unwrap_or((None, None, None));

    let (header_spans, body_spans) = get_header_and_body_spans(line_spans, first_empty_line_idx);

    let body_span = get_span_extent_from_spans(body_spans);

    Ok(PartialHttpRequest::parsed(
        input,
        method,
        uri,
        http_version,
        header_spans,
        body_span,
    ))
}

/// Parse the first line of an HTTP request message
fn parse_first_line(first_line: &str) -> FirstLineParts {
    let mut parts = vec![];
    let mut last_end = 0;

    for (i, c) in first_line.char_indices() {
        if c.is_whitespace() {
            if i > last_end {
                parts.push(last_end..i);
            }
            last_end = i + 1;
        }
    }

    if last_end < first_line.len() {
        parts.push(last_end..first_line.len());
    }

    let method_span = parts.get(0).cloned();
    let uri_span = parts.get(1).cloned();
    let http_version_span = parts.get(2).cloned();

    (method_span, uri_span, http_version_span)
}

fn get_header_and_body_spans(
    line_spans: Vec<Range<usize>>,
    first_empty_line_idx: Option<usize>,
) -> (Vec<Range<usize>>, Option<Vec<Range<usize>>>) {
    let (header_spans, body_spans) = match first_empty_line_idx {
        Some(idx) => {
            let header_spans = line_spans.clone()[1..idx].to_vec();
            let body_spans = Some(line_spans.clone()[idx..].to_vec());

            (header_spans, body_spans)
        }
        None => {
            let header_spans = line_spans.clone()[1..].to_vec();
            let body_spans = None;

            (header_spans, body_spans)
        }
    };
    (header_spans, body_spans)
}

fn get_span_extent_from_spans(body_spans: Option<Vec<Range<usize>>>) -> Option<Range<usize>> {
    let body_span = body_spans.and_then(|spans| {
        if spans.is_empty() {
            return None;
        }

        let first = spans.first().unwrap();
        let last = spans.last().unwrap();

        Some(first.start + 1..last.end)
    });
    body_span
}

#[cfg(test)]
mod tests {
    use crate::{
        error::Error,
        models::{HttpRequest, PartialHttpRequest},
    };

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_method_span() {
        PartialHttpRequest::parsed("", Some(1..2), None, None, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_method_span() {
        PartialHttpRequest::parsed("", Some(2..1), None, None, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_uri_span() {
        PartialHttpRequest::parsed("", None, Some(1..2), None, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_uri_span() {
        PartialHttpRequest::parsed("", None, Some(2..1), None, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_method_span_overlaps_uri_span() {
        PartialHttpRequest::parsed(
            "GET https://example.com",
            Some(0..3),
            Some(2..10),
            None,
            vec![],
            None,
        );
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_http_version_span() {
        PartialHttpRequest::parsed("", None, None, Some(1..2), vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_http_version_span() {
        PartialHttpRequest::parsed("", None, None, Some(2..1), vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_header_span() {
        PartialHttpRequest::parsed("", None, None, None, vec![1..2], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_header_span() {
        PartialHttpRequest::parsed("", None, None, None, vec![2..1], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_body_span() {
        PartialHttpRequest::parsed("", None, None, None, vec![], Some(1..2));
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_body_span() {
        PartialHttpRequest::parsed("", None, None, None, vec![], Some(2..1));
    }

    #[test]
    fn implements_default() {
        let partial = PartialHttpRequest::default();

        assert_eq!(
            PartialHttpRequest::parsed(
                "GET https://example.com HTTP/1.1",
                Some(0..3),
                Some(4..23),
                Some(24..32),
                vec![],
                None
            ),
            partial
        );

        assert_eq!("GET https://example.com HTTP/1.1", partial.message());

        let request: Result<HttpRequest, Error> = partial.try_into();

        assert_eq!(
            Ok(HttpRequest {
                uri: "https://example.com".into(),
                method: "GET".into(),
                http_version: "HTTP/1.1".into(),
                headers: vec![],
                body: None
            }),
            request
        );
    }
}
