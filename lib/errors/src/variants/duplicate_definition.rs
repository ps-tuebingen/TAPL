use std::fmt;

#[derive(Debug)]
pub struct DuplicateDefinition {
    name: String,
}

impl DuplicateDefinition {
    #[must_use] pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl fmt::Display for DuplicateDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} was defined multiple times", self.name)
    }
}

impl std::error::Error for DuplicateDefinition {}
