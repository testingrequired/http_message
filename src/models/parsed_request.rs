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
pub struct ParsedHttpRequest<'http_message> {
    message: &'http_message str,
    method: Range<usize>,
    uri: Range<usize>,
    http_version: Range<usize>,
    headers: Vec<Range<usize>>,
    body: Option<Range<usize>>,
}

impl<'http_message> fmt::Display for ParsedHttpRequest<'http_message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl<'http_message> ParsedHttpRequest<'http_message> {
    pub fn from_str(message: &'http_message str) -> Result<Self, Error> {
        parse_request(message, parse_first_line)
    }

    pub fn parsed(
        message: &'http_message str,
        method: Range<usize>,
        uri: Range<usize>,
        http_version: Range<usize>,
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
        {
            assert!(self.method.start < self.method.end);
            assert_text_span(self.message(), &self.method);
        };

        {
            assert!(self.uri.start < self.uri.end);
            assert_text_span(self.message(), &self.uri);

            if !(self.method.start < self.uri.start && self.method.end < self.uri.start) {
                panic!(
                    "uri {:?} and method {:?} spans conflict",
                    self.uri, self.method
                );
            }
        };

        {
            assert!(self.http_version.start < self.http_version.end);
            assert_text_span(self.message(), &self.http_version);

            if !(self.uri.start < self.http_version.start && self.uri.end < self.http_version.start)
            {
                panic!(
                    "http version {:?} and uri {:?} spans conflict",
                    self.http_version, self.uri
                );
            }
        };

        for span in self.header_spans().iter() {
            assert!(span.start < span.end);
            assert_text_span(self.message(), span);
        }

        self.body.as_ref().inspect(|span| {
            assert!(
                span.start <= span.end,
                "body span {:?} is not contained within message {}",
                span,
                self.message()
            );
            assert_text_span(self.message(), span);
        });
    }

    /// Get the original HTTP request message text
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the text span of the uri, if defined
    pub fn uri_span(&self) -> &Range<usize> {
        &self.uri
    }

    /// Get the string text of the uri, if defined
    pub fn uri_str(&self) -> &str {
        self.slice_message(&self.uri)
    }

    /// Get the text span of the method, if defined
    pub fn method_span(&self) -> &Range<usize> {
        &self.method
    }

    /// Get the string text of the method, if defined
    pub fn method_str(&self) -> &str {
        self.slice_message(&self.method)
    }

    /// Get the text span of the http version, if defined
    pub fn http_version_span(&self) -> &Range<usize> {
        &self.http_version
    }

    /// Get the string text of the http version, if defined
    pub fn http_version_str(&self) -> &str {
        self.slice_message(&self.http_version)
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

impl<'http_message> Default for ParsedHttpRequest<'http_message> {
    fn default() -> Self {
        Self::from_str("GET https://example.com HTTP/1.1\n\n").unwrap()
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
) -> Result<ParsedHttpRequest<'http_message>, Error>
where
    F: Fn(&str) -> FirstLineParts,
{
    if input.trim().is_empty() {
        return Err(Error::EmptyHttpMessage);
    }

    let line_spans = get_line_spans(input);

    let first_empty_line_idx = line_spans
        .iter()
        .position(|span| span.len() == 1)
        .expect("should have at least one empty line in HTTP request");

    let first_line = line_spans.first().unwrap();

    let (method, uri, http_version) = parse_first_line(&input[first_line.clone()]);

    let method = method.unwrap();
    let uri = uri.unwrap();
    let http_version = http_version.unwrap();

    let (header_spans, body_spans) = get_header_and_body_spans(line_spans, first_empty_line_idx);

    let body_span = get_span_extent_from_spans(body_spans);

    Ok(ParsedHttpRequest::parsed(
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
    first_empty_line_idx: usize,
) -> (Vec<Range<usize>>, Option<Vec<Range<usize>>>) {
    let header_spans = line_spans.clone()[1..first_empty_line_idx].to_vec();
    let body_spans = Some(line_spans.clone()[first_empty_line_idx..].to_vec());

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

    if let Some(body_span) = &body_span
        && body_span.is_empty()
    {
        return None;
    }

    body_span
}

#[cfg(test)]
mod tests {
    use crate::models::{HttpRequest, ParsedHttpRequest};

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_method_span() {
        ParsedHttpRequest::parsed("", 1..2, 0..0, 0..0, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_method_span() {
        ParsedHttpRequest::parsed("", 2..1, 0..0, 0..0, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_uri_span() {
        ParsedHttpRequest::parsed("", 0..0, 1..2, 0..0, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_uri_span() {
        ParsedHttpRequest::parsed("", 0..0, 2..1, 0..0, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_method_span_overlaps_uri_span() {
        ParsedHttpRequest::parsed("GET https://example.com", 0..3, 2..10, 0..0, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_http_version_span() {
        ParsedHttpRequest::parsed("", 0..0, 0..0, 1..2, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_http_version_span() {
        ParsedHttpRequest::parsed("", 0..0, 0..0, 2..1, vec![], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_header_span() {
        ParsedHttpRequest::parsed("", 0..0, 0..0, 0..0, vec![1..2], None);
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_header_span() {
        ParsedHttpRequest::parsed("", 0..0, 0..0, 0..0, vec![2..1], None);
    }

    #[test]
    #[should_panic]
    fn verifies_out_of_bounds_body_span() {
        ParsedHttpRequest::parsed("", 0..0, 0..0, 0..0, vec![], Some(1..2));
    }

    #[test]
    #[should_panic]
    fn verifies_inverted_body_span() {
        ParsedHttpRequest::parsed("", 0..0, 0..0, 0..0, vec![], Some(2..1));
    }

    #[test]
    fn implements_default() {
        let parsed = ParsedHttpRequest::default();

        assert_eq!(
            ParsedHttpRequest::parsed(
                "GET https://example.com HTTP/1.1\n\n",
                0..3,
                4..23,
                24..32,
                vec![],
                None
            ),
            parsed
        );

        assert_eq!("GET https://example.com HTTP/1.1\n\n", parsed.message());

        let request: HttpRequest = parsed.into();

        assert_eq!(
            HttpRequest {
                uri: "https://example.com".into(),
                method: "GET".into(),
                http_version: "HTTP/1.1".into(),
                headers: vec![],
                body: None
            },
            request
        );
    }
}
