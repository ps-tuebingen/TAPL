use std::fmt;

/// Language does not exist
#[derive(Debug)]
pub struct UndefinedLanguage {
    /// The language
    lang: String,
}

impl UndefinedLanguage {
    /// Create a new error from the language
    #[must_use]
    pub fn new(lang: &str) -> Self {
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
