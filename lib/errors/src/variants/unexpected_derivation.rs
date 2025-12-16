use std::fmt;

/// Got the wrong derivation ([`derivations::Derivation`])
#[derive(Debug)]
pub struct UnexpectedDerivation {
    /// Found derivation (as string)
    found: String,
    /// Expected derivation (as string)
    expected: String,
}

impl UnexpectedDerivation {
    /// Create a new error from found and expected
    #[must_use]
    pub fn new(found: &str, expected: &str) -> Self {
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
