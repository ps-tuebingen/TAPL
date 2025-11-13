use std::fmt;

#[derive(Debug)]
pub struct RemainingInput {
    remaining: String,
}

impl RemainingInput {
    #[must_use] pub fn new(remaining: &str) -> Self {
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
