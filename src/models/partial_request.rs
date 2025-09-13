use std::{ops::Range, str::FromStr};

use crate::{error::Error, parse};

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

impl FromStr for PartialHttpRequest {
    type Err = Error;

    /// Parse a string in to a partial request
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse_request(s)
    }
}
