use core::fmt;

use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct Uri(Url);

impl Uri {
    pub fn new(uri: &str) -> Self {
        Self(Url::parse(uri).expect("should be a valid url"))
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
