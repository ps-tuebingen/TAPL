use std::fmt;

#[derive(Debug)]
pub struct NameMismatch {
    found: String,
    expected: String,
}

impl NameMismatch {
    pub fn new(found: &str, expected: &str) -> NameMismatch {
        NameMismatch {
            found: found.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl fmt::Display for NameMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected name: {}, expected: {}",
            self.found, self.expected
        )
    }
}

impl std::error::Error for NameMismatch {}
