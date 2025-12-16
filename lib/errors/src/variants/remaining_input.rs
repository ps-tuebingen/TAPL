use std::fmt;

/// Error for extra input during parsing
#[derive(Debug)]
pub struct RemainingInput {
    /// Remaining input after parsing
    remaining: String,
}

impl RemainingInput {
    /// Create a new error from remaining input
    #[must_use]
    pub fn new(remaining: &str) -> Self {
        Self {
            remaining: remaining.to_owned(),
        }
    }
}

impl fmt::Display for RemainingInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Remaining Input {}", self.remaining)
    }
}

impl std::error::Error for RemainingInput {}
