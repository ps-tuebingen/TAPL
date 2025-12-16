use std::fmt;

/// Error during toml parsing
#[derive(Debug)]
pub struct Toml {
    /// Toml source string
    source: String,
    /// Error message
    /// usually [`basic_toml::Error`]
    msg: String,
}

impl Toml {
    /// Create an new error from source and error
    pub fn new<E>(src: &str, err: E) -> Self
    where
        E: std::error::Error,
    {
        Self {
            source: src.to_owned(),
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse toml: {}\n\t{}", self.msg, self.source)
    }
}
