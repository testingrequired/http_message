use crate::models::partial_request::PartialHttpRequest;

pub mod models;

pub fn parse(input: &str) -> PartialHttpRequest {
    let input_lines: Vec<_> = input.lines().collect();

    if input_lines.len() == 0 {
        panic!("Requests can't be empty");
    }

    let first_line = input_lines.first().cloned().unwrap();
    let (method, uri) = parse_first_line(first_line);

    let empty_line_index = input_lines.iter().position(|line| line.is_empty());

    let headers = empty_line_index
        .map(|index| &input_lines[1..index])
        .unwrap_or(&input_lines[1..])
        .iter()
        .map(|l| l.to_string())
        .collect();

    let body = match &empty_line_index {
        Some(index) => Some(input_lines.split_at(*index + 1).1.join("\n")),
        None => None,
    };

    PartialHttpRequest {
        uri,
        method,
        headers,
        body,
    }
}

fn parse_first_line(first_line: &str) -> (String, String) {
    let parts: Vec<_> = first_line.split_whitespace().collect();

    if parts.len() == 0 {
        panic!("Request first line can't be empty");
    }

    let method = parts.first().cloned().unwrap();
    let uri = parts.get(1).cloned().unwrap();

    (method.to_string(), uri.to_string())
}
