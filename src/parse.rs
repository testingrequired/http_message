use std::ops::Range;

use crate::{models::partial_request::PartialHttpRequest, span::get_line_spans};

pub(crate) fn parse_request(input: &str) -> PartialHttpRequest {
    let line_spans = get_line_spans(input);

    let first_empty_line = line_spans.iter().position(|span| span.len() == 1);

    let (method, uri, http_version) = line_spans
        .first()
        .map(|span| parse_first_line(&input[span.clone()]))
        .unwrap_or((None, None, None));

    let (header_spans, body_spans) = match first_empty_line {
        Some(first_empty_line_idx) => {
            let header_spans = line_spans.clone()[1..first_empty_line_idx].to_vec();

            (
                header_spans,
                Some(line_spans.clone()[first_empty_line_idx..].to_vec()),
            )
        }
        None => {
            let header_spans = line_spans.clone()[1..].to_vec();

            (header_spans, None)
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

    PartialHttpRequest::new(input, uri, method, http_version, header_spans, body_span)
}

fn parse_first_line(
    first_line: &str,
) -> (
    Option<Range<usize>>,
    Option<Range<usize>>,
    Option<Range<usize>>,
) {
    let mut start = 0usize;

    let parts: Vec<_> = first_line.split_whitespace().collect();

    if parts.len() == 0 {
        panic!("Request first line can't be empty");
    }

    let method = parts.first().cloned().unwrap();
    let method_span = start..start + method.len();
    start = method_span.end + 1;

    let uri = parts.get(1).cloned().unwrap();
    let uri_span = start..start + uri.len();
    start = uri_span.end + 1;

    let http_version = parts.get(2).cloned();
    let http_version_span = http_version.map(|http_version| start..start + http_version.len());

    (Some(method_span), Some(uri_span), http_version_span)
}
