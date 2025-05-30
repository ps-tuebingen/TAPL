use crate::{kinds::Kind, types::Type};
use std::fmt;

pub mod kinds;
pub use kinds::{KindKind, TypeKind};

#[derive(Debug)]
pub enum Error {
    KindMismatch { found: Kind, expected: KindKind },
    TypeMismatch { found: TypeKind, expected: TypeKind },
}

impl Error {
    pub fn kind(found: Kind, expected: KindKind) -> Error {
        Error::KindMismatch { found, expected }
    }

    pub fn ty<T>(found: &T, expected: TypeKind) -> Error
    where
        T: Type,
    {
        Error::TypeMismatch {
            found: found.knd(),
            expected,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::KindMismatch { found, expected } => {
                write!(f, "Kind Mismatch:\n\texpected: {expected}\n\tfound {found}")
            }
            Error::TypeMismatch { found, expected } => {
                write!(f, "Type Mismatch:\n\texpected: {expected}, found: {found}")
            }
        }
    }
}

impl std::error::Error for Error {}
