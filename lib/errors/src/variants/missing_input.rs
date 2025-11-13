use std::fmt;

#[derive(Debug)]
pub struct MissingInput {
    input: String,
}

impl MissingInput {
    #[must_use] pub fn new(input: &str) -> Self {
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
