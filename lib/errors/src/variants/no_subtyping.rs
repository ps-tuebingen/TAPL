use std::fmt;

#[derive(Debug)]
pub struct NoSubtyping {
    lang: String,
}

impl NoSubtyping {
    #[must_use] pub fn new(lang: &str) -> Self {
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
