use crate::models::{
    body::{HttpBody, PossibleHttpBody},
    headers::HttpHeader,
    partial_request::PartialHttpRequest,
    uri::Uri,
    version::HttpVersion,
};

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    Other(String),
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::Other(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub uri: Uri,
    pub method: HttpMethod,
    pub http_version: HttpVersion,
    pub headers: Vec<HttpHeader>,
    pub body: PossibleHttpBody,
}

impl HttpRequest {
    pub fn get(uri: &str, headers: Vec<HttpHeader>) -> Self {
        Self {
            uri: Uri::new(uri),
            method: HttpMethod::GET,
            http_version: Default::default(),
            headers,
            body: None,
        }
    }

    pub fn post(uri: &str, headers: Vec<HttpHeader>, body: PossibleHttpBody) -> Self {
        Self {
            uri: Uri::new(uri),
            method: HttpMethod::POST,
            headers,
            body,
            http_version: Default::default(),
        }
    }

    pub fn headers(&self) -> &Vec<HttpHeader> {
        &self.headers
    }

    pub fn get_header(&self, key: &str) -> Option<&HttpHeader> {
        self.headers.iter().find(|header| header.key() == key)
    }

    /// Set or update header by key
    pub fn set_header(&mut self, key: &str, value: &str) {
        let existing_header: Option<&mut HttpHeader> = self.get_header_mut(key);
        if let Some(header) = existing_header {
            *header = (key, value).into();
        } else {
            self.headers.push((key, value).into());
        }
    }

    pub fn get_header_mut(&mut self, key: &str) -> Option<&mut HttpHeader> {
        self.headers.iter_mut().find(|header| header.key() == key)
    }
}

impl HttpBody for HttpRequest {
    fn get_body(&self) -> &PossibleHttpBody {
        &self.body
    }

    fn set_body(&mut self, value: PossibleHttpBody) {
        self.body = value;
    }
}

impl From<PartialHttpRequest> for HttpRequest {
    fn from(value: PartialHttpRequest) -> Self {
        Self {
            uri: Uri::new(&value.uri().expect("should have a uri")),
            method: value
                .method()
                .expect("should have a method")
                .as_str()
                .into(),
            headers: value
                .headers()
                .iter()
                .map(|header| header.as_str().into())
                .collect(),
            body: value.get_body(),
            http_version: value
                .http_version()
                .map(|version| HttpVersion::new(&version))
                .or(Some("HTTP/1.1".into()))
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod from_partial_request_tests {
    use crate::models::{partial_request::PartialHttpRequest, request::HttpRequest, uri::Uri};

    use pretty_assertions::assert_eq;

    #[test]
    fn from_partial_request_get() {
        let partial_request = PartialHttpRequest::new(
            r#"
        GET https://example.com HTTP/1.1
        x-api-key: abc123
        "#,
            Some(13..32),
            Some(9..12),
            Some(33..41),
            vec![50..67],
            None,
        );

        let request: HttpRequest = partial_request.into();

        assert_eq!(
            HttpRequest {
                uri: Uri::new("https://example.com"),
                method: "GET".into(),
                http_version: "HTTP/1.1".into(),
                headers: vec!["x-api-key: abc123".into()],
                body: None,
            },
            request
        );
    }

    #[test]
    #[ignore = "TODO"]
    fn from_partial_request_post() {
        let partial_request = PartialHttpRequest::new(
            r#"
        POST https://example.com HTTP/1.1
        x-api-key: abc123

        request body"#,
            Some(13..32),
            Some(9..12),
            Some(33..41),
            vec![50..67],
            None,
        );

        let request: HttpRequest = partial_request.into();

        assert_eq!(
            HttpRequest {
                uri: Uri::new("https://example.com"),
                method: "POST".into(),
                http_version: "HTTP/1.1".into(),
                headers: vec!["x-api-key: abc123".into()],
                body: Some("request body".to_string()),
            },
            request
        );
    }
}

#[cfg(test)]
mod request_tests {
    use crate::models::{
        body::HttpBody,
        headers::HttpHeader,
        request::{HttpMethod, HttpRequest},
    };

    #[test]
    fn test_request_with_headers() {
        let mut request = HttpRequest::get(
            "https://example.com",
            vec!["Authorization: Bearer token".into()],
        );

        request.set_header("X-API-Key", "API Key");

        assert_eq!(
            request.get_header("Authorization").unwrap().value(),
            "Bearer token"
        );

        assert_eq!(request.get_header("X-API-Key").unwrap().value(), "API Key");

        let expected_headers_in_order: Vec<HttpHeader> = vec![
            "Authorization: Bearer token".into(),
            ("X-API-Key", "API Key").into(),
        ];

        assert_eq!(&expected_headers_in_order, request.headers())
    }

    #[test]
    fn test_request_get() {
        let request = HttpRequest::get(
            "https://example.com",
            vec!["User-Agent: curl/7.64.1".into()],
        );
        assert_eq!(request.method, HttpMethod::GET);
        assert!(request.body.is_none());

        let expected_headers_in_order: Vec<HttpHeader> = vec!["User-Agent: curl/7.64.1".into()];

        assert_eq!(expected_headers_in_order, *request.headers())
    }

    #[test]
    fn test_request_post() {
        let headers = vec!["Content-Type: application/json".into()];
        let body = Some("{\"key\": \"value\"}".to_string());
        let request = HttpRequest::post("https://example.com", headers, body);
        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(
            request.get_body(),
            &Some("{\"key\": \"value\"}".to_string())
        );

        let expected_headers_in_order: Vec<HttpHeader> =
            vec!["Content-Type: application/json".into()];

        assert_eq!(expected_headers_in_order, *request.headers())
    }
}
