use super::kinds::ValueKind;
use std::fmt;

#[derive(Debug)]
pub struct ValueMismatch {
    found: ValueKind,
    expected: ValueKind,
}

impl ValueMismatch {
    pub fn new(found: ValueKind, expected: ValueKind) -> ValueMismatch {
        ValueMismatch { found, expected }
    }
}

impl fmt::Display for ValueMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value Mismatch:\n\tfound {},\n\texpected {}",
            self.found, self.expected
        )
    }
}

impl std::error::Error for ValueMismatch {}
