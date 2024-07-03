pub mod config;
pub mod regex;
pub mod scanner;
pub mod secret_type;

pub use config::Config;
pub use scanner::{FoundSecret, Scanner};
pub use secret_type::SecretType;

extern crate regex as extern_regex;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinosCodexError {
    #[error("Failed to load configuration: {0}")]
    ConfigLoadError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Invalid regex: {0}")]
    RegexError(String),
    // RegexError(#[from] extern_regex::Error),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub fn create_scanner(config_dir: &str) -> Result<Scanner, MinosCodexError> {
    let config = Config::load_from_directory(config_dir)?;
    Ok(Scanner::new(config))
}

impl From<extern_regex::Error> for MinosCodexError {
    fn from(err: extern_regex::Error) -> Self {
        match err {
            extern_regex::Error::Syntax(msg) => {
                MinosCodexError::RegexError(format!("Syntax error: {}", msg))
            }
            extern_regex::Error::CompiledTooBig(size) => {
                MinosCodexError::RegexError(format!("Compiled pattern too big: {} bytes", size))
            }
            _ => MinosCodexError::RegexError("Unhandled regex error".to_string()),
        }
    }
}
