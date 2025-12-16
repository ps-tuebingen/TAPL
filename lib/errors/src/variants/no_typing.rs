use std::fmt;

/// Language has no types
#[derive(Debug)]
pub struct NoTyping {
    /// The Language
    lang: String,
}

impl NoTyping {
    /// Create a new error from the language
    #[must_use]
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.to_owned(),
        }
    }
}

impl fmt::Display for NoTyping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Langauge {} has no types", self.lang)
    }
}

impl std::error::Error for NoTyping {}
