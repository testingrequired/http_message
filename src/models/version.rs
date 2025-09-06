use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpVersion(String);

impl HttpVersion {
    fn is_prefixed(&self) -> bool {
        self.0.starts_with("HTTP/")
    }
}

impl Default for HttpVersion {
    fn default() -> Self {
        Self("HTTP/1.1".to_string())
    }
}

impl From<&str> for HttpVersion {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_prefixed() {
            write!(f, "{}", self.0)
        } else {
            write!(f, "HTTP/{}", self.0)
        }
    }
}

#[cfg(test)]
mod http_version_tests {
    use super::*;

    #[test]
    fn test_is_prefixed_with_http_prefix() {
        let version = HttpVersion("HTTP/1.1".to_string());
        assert!(version.is_prefixed());
    }

    #[test]
    fn test_is_prefixed_without_http_prefix() {
        let version = HttpVersion("1.1".to_string());
        assert!(!version.is_prefixed());
    }

    #[test]
    fn test_default_value() {
        let version = HttpVersion::default();
        assert_eq!(version.to_string(), "HTTP/1.1");
    }

    #[test]
    fn test_from_str_with_http_prefix() {
        let version = HttpVersion::from("HTTP/1.1");
        assert_eq!(version.to_string(), "HTTP/1.1");
    }

    #[test]
    fn test_from_str_without_http_prefix() {
        let version = HttpVersion::from("1.1");
        assert_eq!(version.to_string(), "HTTP/1.1");
    }
}
