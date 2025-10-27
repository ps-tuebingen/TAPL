use std::fmt;

#[derive(Debug)]
pub struct NoKinding {
    lang: String,
}

impl NoKinding {
    pub fn new(lang: &str) -> NoKinding {
        NoKinding {
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
