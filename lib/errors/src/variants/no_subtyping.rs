use std::fmt;

/// Language has no subtypes
#[derive(Debug)]
pub struct NoSubtyping {
    /// The Language
    lang: String,
}

impl NoSubtyping {
    /// Create a new error from the language
    #[must_use]
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.to_owned(),
        }
    }
}

impl fmt::Display for NoSubtyping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Langauge {} has no subtypes", self.lang)
    }
}

impl std::error::Error for NoSubtyping {}
