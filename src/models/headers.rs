use core::fmt;

/// An HTTP header key & value
///
/// ```skip
/// GET example.com HTTP/1.1
/// key: value
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeader(String, String);

impl HttpHeader {
    pub fn new(key: &str, value: &str) -> Self {
        Self(key.to_string(), value.to_string())
    }

    pub fn key(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        &self.1
    }
}

impl fmt::Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key(), self.value())
    }
}

impl From<(&str, &str)> for HttpHeader {
    fn from(value: (&str, &str)) -> Self {
        HttpHeader::new(value.0, value.1)
    }
}

impl From<&str> for HttpHeader {
    fn from(value: &str) -> Self {
        let index = value
            .chars()
            .position(|c| c == ':')
            .expect("should find ':' in header string");

        let (key, value) = value.split_at(index);

        let value = value[1..].trim();

        (key, value).into()
    }
}

pub trait HttpHeaders {
    type Header;

    fn headers(&self) -> Vec<Self::Header>;
    fn get_header(&self, key: &str) -> Option<Self::Header>;
    fn get_header_mut(&mut self, key: &str) -> Option<&mut Self::Header>;
    fn set_header(&mut self, key: &str, value: &str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header_new() {
        let header = HttpHeader::new("Content-Type", "application/json");
        assert_eq!(header.0, "Content-Type");
        assert_eq!(header.1, "application/json");
    }

    #[test]
    fn test_http_header_display() {
        let header = HttpHeader::new("Content-Type", "application/json");
        assert_eq!(format!("{}", header), "Content-Type: application/json");
    }

    #[test]
    fn test_http_header_from_tuple() {
        let header: HttpHeader = ("Content-Type", "application/json").into();
        assert_eq!(header.key(), "Content-Type");
        assert_eq!(header.value(), "application/json");
    }

    #[test]
    fn test_http_header_from_str() {
        let header: HttpHeader = "Content-Type: application/json".into();
        assert_eq!(header.key(), "Content-Type");
        assert_eq!(header.value(), "application/json");
    }
}
