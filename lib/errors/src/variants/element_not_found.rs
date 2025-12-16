use std::fmt;

/// Error looking up html element
#[derive(Debug)]
pub struct ElementNotFound {
    /// Id of the element
    id: String,
}

impl ElementNotFound {
    /// Create a new error from id
    #[must_use]
    pub fn new(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl fmt::Display for ElementNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find element with id {}", self.id)
    }
}

impl std::error::Error for ElementNotFound {}
