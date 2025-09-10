use std::ops::Range;

use crate::{models::partial_request::PartialHttpRequest, span::LineSpans};

pub fn parse_request(input: &str) -> PartialHttpRequest {
    let lines_count = input.lines().count();

    let binding: LineSpans = input.lines().fold(vec![None], |mut acc, line| {
        let prev = acc.last().unwrap();

        if line.trim().is_empty() {
            acc.push(None);

            return acc;
        }

        match prev {
            Some(prev) => {
                // Last line
                if acc.len() == lines_count {
                    acc.push(Some(prev.end..prev.end + line.len()));
                // Other lines
                } else {
                    acc.push(Some(prev.end..prev.end + line.len()));
                }
            }
            None => {
                // First line
                acc.push(Some(0..line.len()))
            }
        }

        acc
    });

    let line_spans: Vec<Range<usize>> = binding
        .iter()
        .skip(1)
        .map(|span| span.clone().unwrap_or(0..0))
        .collect();

    dbg!(&line_spans);

    if line_spans.len() == 0 {
        panic!("Requests can't be empty");
    }

    let first_line = line_spans
        .first()
        .map(|first_line_span| &input[first_line_span.start..first_line_span.end])
        .unwrap();

    let (method, uri, http_version) = parse_first_line(first_line);

    let empty_line_index = line_spans
        .iter()
        .position(|line_span| line_span.start == line_span.end);

    eprintln!("Empty line index: {:?}", empty_line_index);

    let headers: Vec<_> = empty_line_index
        .map(|index| &line_spans[1..index + 1])
        .unwrap_or(&line_spans[1..])
        .into_iter()
        .map(|x| (*x).clone())
        .collect();

    let body = match &empty_line_index {
        Some(index) => Some(line_spans[*index + 1].start..input.len()),
        None => None,
    };

    PartialHttpRequest::new(input, uri, method, http_version, headers.to_vec(), body)
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
