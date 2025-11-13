use std::fmt;

#[derive(Debug)]
pub struct Toml {
    source: String,
    msg: String,
}

impl Toml {
    pub fn new<E>(src: &str, err: E) -> Self
    where
        E: std::error::Error,
    {
        Self {
            source: src.to_owned(),
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse toml: {}\n\t{}", self.msg, self.source)
    }
}
