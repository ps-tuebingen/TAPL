use std::fmt;

#[derive(Debug)]
pub struct UnexpectedRule {
    found: String,
    expected: String,
}
impl UnexpectedRule {
    pub fn new(found: &str, expected: &str) -> UnexpectedRule {
        UnexpectedRule {
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
