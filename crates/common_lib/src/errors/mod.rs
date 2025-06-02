use std::fmt;

pub mod kinds;
pub use kinds::{KindKind, TypeKind};

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

impl fmt::Display for TypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type Mismatch:\n\texpected: {}, found: {}",
            self.expected, self.found
        )
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
