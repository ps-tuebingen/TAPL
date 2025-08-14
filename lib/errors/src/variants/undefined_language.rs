use std::fmt;

#[derive(Debug)]
pub struct UndefinedLanguage {
    lang: String,
}

impl UndefinedLanguage {
    pub fn new(lang: &str) -> UndefinedLanguage {
        UndefinedLanguage {
            lang: lang.to_owned(),
        }
    }
}

impl fmt::Display for UndefinedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined language {}", self.lang)
    }
}

impl std::error::Error for UndefinedLanguage {}
