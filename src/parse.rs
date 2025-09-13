use std::ops::Range;

use crate::{error::Error, models::partial_request::PartialHttpRequest, span::get_line_spans};

pub(crate) fn parse_request(input: &str) -> Result<PartialHttpRequest, Error> {
    let line_spans = get_line_spans(input);

    let first_empty_line_idx = line_spans.iter().position(|span| span.len() == 1);

    let first_line = line_spans.first();

    let (method, uri, http_version) = first_line
        .map(|span| &input[span.clone()])
        .map(parse_first_line)
        .unwrap_or((None, None, None));

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

    let body_span = body_spans.and_then(|spans| {
        if spans.is_empty() {
            return None;
        }

        let first = spans.first().unwrap();
        let last = spans.last().unwrap();

        Some(first.start + 1..last.end)
    });

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
fn parse_first_line(
    first_line: &str,
) -> (
    Option<Range<usize>>,
    Option<Range<usize>>,
    Option<Range<usize>>,
) {
    let parts: Vec<_> = first_line.split_whitespace().collect();

    let mut start = 0;

    let mut get_span = |part: &str| {
        let span = start..start + part.len();
        start = span.end + 1;
        span
    };

    let method_span = parts.get(0).map(|&method| get_span(method));
    let uri_span = parts.get(1).map(|&uri| get_span(uri));
    let http_version_span = parts.get(2).map(|&version| get_span(version));

    (method_span, uri_span, http_version_span)
}
