use crate::models::{ParsedHttpRequest, PartialHttpRequest};

pub mod error;
pub mod models;
pub mod span;

/// Parse a partial HTTP request message string in to [PartialHttpRequest]
pub fn parse_partial_request(input: &str) -> Result<PartialHttpRequest<'_>, error::Error> {
    PartialHttpRequest::parse(input)
}

/// Parse a spec compliant HTTP request message string in to [ParsedHttpRequest]
pub fn parse_request(input: &str) -> Result<ParsedHttpRequest<'_>, error::Error> {
    ParsedHttpRequest::parse(input)
}
