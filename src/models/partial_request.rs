use std::ops::Range;

use crate::models::body::PossibleHttpBody;

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

    pub fn uri(&self) -> Option<String> {
        self.uri
            .as_ref()
            .map(|span| self.message[span.start..span.end].to_string())
    }

    pub fn method(&self) -> Option<String> {
        self.method
            .as_ref()
            .map(|span| self.message[span.start..span.end].to_string())
    }

    pub fn http_version(&self) -> Option<String> {
        self.http_version
            .as_ref()
            .map(|span| self.message[span.start..span.end].to_string())
    }

    pub fn headers(&self) -> Vec<String> {
        self.headers
            .iter()
            .map(|span| self.message[span.start..span.end].to_string())
            .collect()
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers
            .clone()
            .into_iter()
            .find(|span| self.message[span.start..span.end].starts_with(&format!("{key}:")))
            .map(|span| self.message[span.start..span.end].to_string())
    }

    pub fn get_body(&self) -> PossibleHttpBody {
        self.body
            .as_ref()
            .map(|body| self.message[body.start..body.end].to_string())
    }
}
