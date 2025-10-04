use snafu::prelude::*;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("HTTP Message strings can't be empty"))]
    EmptyHttpMessage,
    #[snafu(display("Required but not found: {key}"))]
    MissingRequired { key: String },
}

impl Error {
    pub fn missing_required(key: &str) -> Self {
        Self::MissingRequired {
            key: key.to_string(),
        }
    }
}
