use std::fmt;

#[derive(Debug)]
pub struct UnknownKeyword {
    kw: String,
}

impl UnknownKeyword {
    pub fn new(kw: &str) -> UnknownKeyword {
        UnknownKeyword { kw: kw.to_owned() }
    }
}

impl fmt::Display for UnknownKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown keyword {}", self.kw)
    }
}

impl std::error::Error for UnknownKeyword {}
