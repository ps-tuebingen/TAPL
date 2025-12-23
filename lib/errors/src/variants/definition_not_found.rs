use std::fmt;
use syntax::Name;

/// Error when a definition is not found in the program
#[derive(Debug)]
pub struct DefinitionNotFound {
    name: Name,
}

impl DefinitionNotFound {
    /// Create a new error for a given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for DefinitionNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find definition {}", self.name)
    }
}

impl std::error::Error for DefinitionNotFound {}
