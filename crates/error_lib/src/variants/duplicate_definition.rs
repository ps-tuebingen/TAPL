use std::fmt;

#[derive(Debug)]
pub struct DuplicateDefinition {
    name: String,
}

impl DuplicateDefinition {
    pub fn new(name: &str) -> DuplicateDefinition {
        DuplicateDefinition {
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
