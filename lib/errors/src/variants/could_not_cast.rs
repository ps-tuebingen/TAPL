use std::fmt;

/// Error during casting html elements
#[derive(Debug)]
pub struct CouldNotCast {
    /// Id of the element
    id: String,
    /// html tag to cast to
    target: String,
}

impl CouldNotCast {
    /// Create a new error from element id and target tag
    #[must_use]
    pub fn new(id: &str, target: &str) -> Self {
        Self {
            id: id.to_owned(),
            target: target.to_owned(),
        }
    }
}

impl fmt::Display for CouldNotCast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not cast element {} to {}", self.id, self.target)
    }
}

impl std::error::Error for CouldNotCast {}
