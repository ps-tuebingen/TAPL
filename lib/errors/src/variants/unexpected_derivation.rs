use std::fmt;

#[derive(Debug)]
pub struct UnexpectedDerivation {
    found: String,
    expected: String,
}

impl UnexpectedDerivation {
    #[must_use] pub fn new(found: &str, expected: &str) -> Self {
        Self {
            found: found.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl fmt::Display for UnexpectedDerivation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected derivation: {}, expected: {}",
            self.found, self.expected
        )
    }
}
