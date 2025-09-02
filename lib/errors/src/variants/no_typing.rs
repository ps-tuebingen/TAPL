use std::fmt;

#[derive(Debug)]
pub struct NoTyping {
    lang: String,
}

impl NoTyping {
    pub fn new(lang: &str) -> NoTyping {
        NoTyping {
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
