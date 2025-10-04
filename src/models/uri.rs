use core::fmt;

use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct Uri(Url);

impl Uri {
    pub fn new(uri: &str) -> Self {
        let uri = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri
        } else {
            &format!("https://{uri}")
        };

        let message = format!("should be a valid url: {uri}");
        Self(Url::parse(uri).unwrap_or_else(|_| panic!("{}", message)))
    }
}

impl Default for Uri {
    fn default() -> Self {
        Self::new("https://example.com")
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
