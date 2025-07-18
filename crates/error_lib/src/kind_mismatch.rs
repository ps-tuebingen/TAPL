use super::KindKind;
use std::fmt;

#[derive(Debug)]
pub struct KindMismatch {
    found: KindKind,
    expected: KindKind,
}

impl KindMismatch {
    pub fn new(found: KindKind, expected: KindKind) -> KindMismatch {
        KindMismatch { found, expected }
    }
}

impl fmt::Display for KindMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Kind Mismatch:\n\texpected: {}\n\tfound {}",
            self.expected, self.found
        )
    }
}
