use super::TypeKind;
use std::fmt;

#[derive(Debug)]
pub struct TypeMismatch {
    found: TypeKind,
    expected: TypeKind,
}

impl TypeMismatch {
    pub fn new(found: TypeKind, expected: TypeKind) -> TypeMismatch {
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
