use crate::models::{
    body::{HttpBody, PossibleHttpBody},
    headers::HttpHeaders,
};

/// A partial HTTP request that might not conform to HTTP spec
///
/// A templated HTTP request message is an example use case.
#[derive(Debug, Clone, PartialEq)]
pub struct PartialHttpRequest {
    pub uri: String,
    pub method: String,
    pub headers: Vec<String>,
    pub body: PossibleHttpBody,
}

impl PartialHttpRequest {
    pub fn get(uri: &str, headers: Vec<String>) -> Self {
        Self {
            uri: uri.to_string(),
            method: String::from("GET"),
            headers,
            body: None,
        }
    }

    pub fn post(uri: &str, headers: Vec<String>, body: PossibleHttpBody) -> Self {
        Self {
            uri: uri.to_string(),
            method: String::from("POST"),
            headers,
            body,
        }
    }
}

impl HttpHeaders for PartialHttpRequest {
    type Header = String;

    fn headers(&self) -> &Vec<Self::Header> {
        &self.headers
    }

    fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.iter().find(|header| header.starts_with(key))
    }

    /// Set or update header by key
    fn set_header(&mut self, key: &str, value: &str) {
        let existing_header: Option<&mut String> = self.get_header_mut(key);
        if let Some(header) = existing_header {
            *header = format!("{key}: {value}");
        } else {
            self.headers.push(format!("{key}: {value}"));
        }
    }

    fn get_header_mut(&mut self, key: &str) -> Option<&mut String> {
        self.headers
            .iter_mut()
            .find(|header| header.starts_with(key))
    }
}

impl HttpBody for PartialHttpRequest {
    fn get_body(&self) -> &PossibleHttpBody {
        &self.body
    }

    fn set_body(&mut self, value: PossibleHttpBody) {
        self.body = value;
    }
}
