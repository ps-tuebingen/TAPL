use std::fmt;

#[derive(Debug)]
pub struct ElementNotFound {
    id: String,
}

impl ElementNotFound {
    pub fn new(id: &str) -> ElementNotFound {
        ElementNotFound { id: id.to_owned() }
    }
}

impl fmt::Display for ElementNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find element with id {}", self.id)
    }
}

impl std::error::Error for ElementNotFound {}
