use crate::Rule;
use std::fmt;

#[derive(Debug)]
pub struct UnexpectedRule {
    found: Rule,
    expected: String,
}
impl UnexpectedRule {
    pub fn new(found: Rule, expected: &str) -> UnexpectedRule {
        UnexpectedRule {
            found,
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
