use std::fmt;

/// Language has no kinds
#[derive(Debug)]
pub struct NoKinding {
    /// Language (as string)
    lang: String,
}

impl NoKinding {
    /// Create a new error from language
    #[must_use]
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.to_owned(),
        }
    }
}

impl fmt::Display for NoKinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Langauge {} has no kinds", self.lang)
    }
}

impl std::error::Error for NoKinding {}
