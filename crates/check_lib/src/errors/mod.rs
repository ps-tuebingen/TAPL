use common::errors::{FreeVariable, NotImplemented, UndefinedLocation};
use std::fmt;

pub mod empty_case;
pub mod free_type_variable;
pub mod name_mismatch;
pub mod not_a_subtype;

pub use empty_case::EmptyCase;
pub use free_type_variable::FreeTypeVariable;
pub use name_mismatch::NameMismatch;
pub use not_a_subtype::NotASubtype;

#[derive(Debug)]
pub enum EnvError {
    NotImplemented(NotImplemented),
    FreeVariable(FreeVariable),
    FreeTypeVariable(FreeTypeVariable),
    UndefinedLocation(UndefinedLocation),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnvError::NotImplemented(ni) => ni.fmt(f),
            EnvError::FreeVariable(fv) => fv.fmt(f),
            EnvError::FreeTypeVariable(ftv) => ftv.fmt(f),
            EnvError::UndefinedLocation(ul) => ul.fmt(f),
        }
    }
}

impl std::error::Error for EnvError {}

impl From<NotImplemented> for EnvError {
    fn from(ni: NotImplemented) -> EnvError {
        EnvError::NotImplemented(ni)
    }
}

impl From<FreeVariable> for EnvError {
    fn from(fv: FreeVariable) -> EnvError {
        EnvError::FreeVariable(fv)
    }
}

impl From<UndefinedLocation> for EnvError {
    fn from(ul: UndefinedLocation) -> EnvError {
        EnvError::UndefinedLocation(ul)
    }
}
