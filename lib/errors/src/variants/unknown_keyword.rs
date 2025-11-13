use std::fmt;

#[derive(Debug)]
pub struct UnknownKeyword {
    kw: String,
}

impl UnknownKeyword {
    #[must_use] pub fn new(kw: &str) -> Self {
        Self { kw: kw.to_owned() }
    }
}

impl fmt::Display for UnknownKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown keyword {}", self.kw)
    }
}

impl std::error::Error for UnknownKeyword {}
