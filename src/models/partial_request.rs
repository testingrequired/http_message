use std::{ops::Range, str::FromStr};

use crate::{error::Error, span::get_line_spans};

/// A partial HTTP request that might not conform to HTTP spec
///
/// A templated HTTP request message is an example use case.
#[derive(Debug, Clone, PartialEq)]
pub struct PartialHttpRequest {
    message: String,
    uri: Option<Range<usize>>,
    method: Option<Range<usize>>,
    http_version: Option<Range<usize>>,
    headers: Vec<Range<usize>>,
    body: Option<Range<usize>>,
}

impl PartialHttpRequest {
    pub fn new(
        message: &str,
        uri: Option<Range<usize>>,
        method: Option<Range<usize>>,
        http_version: Option<Range<usize>>,
        headers: Vec<Range<usize>>,
        body: Option<Range<usize>>,
    ) -> Self {
        Self {
            message: message.to_string(),
            uri,
            method,
            http_version,
            headers,
            body,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn uri_span(&self) -> Option<Range<usize>> {
        self.uri.clone()
    }

    pub fn uri_str(&self) -> Option<&str> {
        self.uri
            .as_ref()
            .map(|span| &self.message[span.start..span.end])
    }

    pub fn method_span(&self) -> Option<Range<usize>> {
        self.method.clone()
    }

    pub fn method_str(&self) -> Option<&str> {
        self.method
            .as_ref()
            .map(|span| &self.message[span.start..span.end])
    }

    pub fn http_version_span(&self) -> Option<Range<usize>> {
        self.http_version.clone()
    }

    pub fn http_version_str(&self) -> Option<&str> {
        self.http_version
            .as_ref()
            .map(|span| &self.message[span.start..span.end])
    }

    pub fn header_spans(&self) -> Vec<Range<usize>> {
        self.headers.clone()
    }

    pub fn header_strs(&self) -> Vec<&str> {
        self.headers
            .iter()
            .map(|span| &self.message[span.start..span.end])
            .collect()
    }

    pub fn header_span(&self, key: &str) -> Option<Range<usize>> {
        self.headers
            .clone()
            .into_iter()
            .find(|span| self.message[span.start..span.end].starts_with(&format!("{key}:")))
    }

    pub fn header_str(&self, key: &str) -> Option<&str> {
        self.headers
            .clone()
            .into_iter()
            .find(|span| self.message[span.start..span.end].starts_with(&format!("{key}:")))
            .map(|span| &self.message[span.start..span.end])
    }

    pub fn body_str(&self) -> Option<&str> {
        self.body
            .as_ref()
            .map(|body| &self.message[body.start..body.end])
    }
}

impl Default for PartialHttpRequest {
    fn default() -> Self {
        Self::from_str("GET https://example.com HTTP/1.1").unwrap()
    }
}

impl FromStr for PartialHttpRequest {
    type Err = Error;

    /// Parse a string in to a partial request
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_request(s, parse_first_line)
    }
}

type FirstLineParts = (
    Option<Range<usize>>,
    Option<Range<usize>>,
    Option<Range<usize>>,
);

fn parse_request<F>(input: &str, parse_first_line: F) -> Result<PartialHttpRequest, Error>
where
    F: Fn(&str) -> FirstLineParts,
{
    if input.trim().is_empty() {
        return Ok(PartialHttpRequest::new(
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

    Ok(PartialHttpRequest::new(
        input,
        uri,
        method,
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
        PartialHttpRequest,
        models::{request::HttpRequest, uri::Uri},
    };

    #[test]
    fn implements_default() {
        let partial = PartialHttpRequest::default();

        assert_eq!(
            PartialHttpRequest::new(
                "GET https://example.com HTTP/1.1",
                Some(4..23),
                Some(0..3),
                Some(24..32),
                vec![],
                None
            ),
            partial
        );

        assert_eq!("GET https://example.com HTTP/1.1", partial.message());

        let request: HttpRequest = partial.into();

        assert_eq!(
            HttpRequest {
                uri: Uri::new("https://example.com"),
                method: "GET".into(),
                http_version: "HTTP/1.1".into(),
                headers: vec![],
                body: None
            },
            request
        );
    }
}
