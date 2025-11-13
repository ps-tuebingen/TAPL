use std::fmt;

#[derive(Debug)]
pub struct UndefinedLanguage {
    lang: String,
}

impl UndefinedLanguage {
    #[must_use] pub fn new(lang: &str) -> Self {
        Self {
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
