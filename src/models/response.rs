use core::fmt;

use crate::models::{
    body::{HttpBody, PossibleHttpBody},
    headers::HttpHeader,
};

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub headers: Vec<HttpHeader>,
    pub body: PossibleHttpBody,
}

impl HttpResponse {
    pub fn new(status_cdoe: HttpStatusCode, headers: Vec<HttpHeader>, body: Option<&str>) -> Self {
        Self {
            status_code: status_cdoe.into(),
            headers,
            body: body.map(|b| b.to_string()),
        }
    }

    pub fn headers(&self) -> &Vec<HttpHeader> {
        &self.headers
    }

    pub fn get_header(&self, key: &str) -> Option<&HttpHeader> {
        self.headers.iter().find(|header| header.key() == key)
    }

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

impl HttpBody for HttpResponse {
    fn get_body(&self) -> &PossibleHttpBody {
        &self.body
    }

    fn set_body(&mut self, value: PossibleHttpBody) {
        self.body = value;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpStatusCode(u16);

impl HttpStatusCode {
    pub fn new(status_code: u16) -> Self {
        Self(status_code)
    }
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for HttpStatusCode {
    fn from(value: u16) -> Self {
        HttpStatusCode(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_code_new() {
        let status_code = HttpStatusCode::new(200);
        assert_eq!(status_code.0, 200);
    }

    #[test]
    fn test_http_status_code_display() {
        let status_code = HttpStatusCode::new(200);
        assert_eq!(format!("{}", status_code), "200");
    }

    #[test]
    fn test_http_status_code_from() {
        let status_code: HttpStatusCode = 200.into();
        assert_eq!(status_code.0, 200);
    }

    #[test]
    fn test_http_response_new() {
        let headers = vec!["Content-Type: application/json".into()];
        let body = Some("{\"message\": \"Hello, world!\"}");
        let response = HttpResponse::new(200.into(), headers.clone(), body);

        assert_eq!(response.status_code.0, 200);
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.headers[0].key(), "Content-Type");
        assert_eq!(response.headers[0].value(), "application/json");
        assert_eq!(response.body, body.map(|b| b.to_string()));
    }

    #[test]
    fn test_http_response_headers() {
        let response = HttpResponse::new(
            200.into(),
            vec!["Content-Type: application/json".into()].clone(),
            None,
        );

        let expected_headers_in_order: Vec<HttpHeader> =
            vec!["Content-Type: application/json".into()];

        assert_eq!(&expected_headers_in_order, response.headers());
    }

    #[test]
    fn test_http_response_get_header() {
        let headers = vec!["Content-Type: application/json".into()];
        let response = HttpResponse::new(200.into(), headers.clone(), None);
        let header = response.get_header("Content-Type");
        assert_eq!(
            Some(&HttpHeader::new("Content-Type", "application/json")),
            header
        );
    }

    #[test]
    fn test_http_response_set_header() {
        let mut response = HttpResponse::new(
            200.into(),
            vec!["Content-Type: application/json".into()],
            None,
        );

        response.set_header("Content-Type", "text/plain");

        let header = response.get_header("Content-Type").unwrap();

        assert_eq!(header.value(), "text/plain");
    }

    // #[test]
    // fn test_http_response_get_header_mut() {
    //     let mut response = HttpResponse::new(
    //         HttpStatusCode::OK,
    //         vec!["Content-Type: application/json".into()],
    //         None,
    //     );

    //     if let Some(header) = response.get_header_mut("Content-Type") {
    //         header.set_value("text/plain");
    //     }

    //     let header = response.get_header("Content-Type").unwrap();
    //     assert_eq!(header.value(), "application/json");
    // }

    #[test]
    fn test_http_response_get_body() {
        let body = Some("{\"message\": \"Hello, world!\"}");
        let response = HttpResponse::new(200.into(), vec![], body);
        assert_eq!(response.get_body(), &body.map(|b| b.to_string()));
    }

    #[test]
    fn test_http_response_set_body() {
        let mut response = HttpResponse::new(200.into(), vec![], None);
        let new_body = Some("{\"message\": \"Goodbye, world!\"}").map(|b| b.to_string());
        response.set_body(new_body.clone());
        assert_eq!(response.get_body(), &new_body);
    }
}
