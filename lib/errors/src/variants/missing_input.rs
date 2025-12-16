use std::fmt;

/// Error for missing parts during parsing
#[derive(Debug)]
pub struct MissingInput {
    /// The input tried to be parsed
    input: String,
}

impl MissingInput {
    /// Create a new error from input
    #[must_use]
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }
}

impl fmt::Display for MissingInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing Input {}", self.input)
    }
}

impl std::error::Error for MissingInput {}
