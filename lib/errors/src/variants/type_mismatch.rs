use std::fmt;

#[derive(Debug)]
pub struct TypeMismatch {
    found: String,
    expected: String,
}

impl TypeMismatch {
    pub fn new(found: String, expected: String) -> TypeMismatch {
        TypeMismatch { found, expected }
    }
}

impl fmt::Display for TypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type Mismatch:\n\texpected: {}, found: {}",
            self.expected, self.found
        )
    }
}
