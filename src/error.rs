use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum PsgcliError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error in {format}: {details}")]
    Parse { format: String, details: String },

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    #[error("Invalid tree structure: {0}")]
    InvalidTree(String),
}

impl PsgcliError {
    pub fn parse(format: impl Into<String>, details: impl Into<String>) -> Self {
        PsgcliError::Parse {
            format: format.into(),
            details: details.into(),
        }
    }
}