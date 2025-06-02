use std::fmt;

#[derive(Debug)]
pub struct UndefinedLanguage {
    name: String,
}

impl UndefinedLanguage {
    pub fn new(name: &str) -> UndefinedLanguage {
        UndefinedLanguage {
            name: name.to_owned(),
        }
    }
}

impl fmt::Display for UndefinedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined language {}", self.name)
    }
}

impl std::error::Error for UndefinedLanguage {}
