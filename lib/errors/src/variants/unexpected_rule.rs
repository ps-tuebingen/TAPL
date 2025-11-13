use std::fmt;

#[derive(Debug)]
pub struct UnexpectedRule {
    found: String,
    expected: String,
}
impl UnexpectedRule {
    #[must_use] pub fn new(found: &str, expected: &str) -> Self {
        Self {
            found: found.to_owned(),
            expected: expected.to_owned(),
        }
    }
}

impl fmt::Display for UnexpectedRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected rule {:?}, expected: {}",
            self.found, self.expected
        )
    }
}

impl std::error::Error for UnexpectedRule {}
