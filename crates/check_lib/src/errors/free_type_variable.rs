use super::EnvError;
use std::fmt;
use syntax::TypeVar;

#[derive(Debug)]
pub struct FreeTypeVariable {
    var: TypeVar,
}

impl FreeTypeVariable {
    pub fn new(var: &TypeVar) -> FreeTypeVariable {
        FreeTypeVariable { var: var.clone() }
    }
}

impl fmt::Display for FreeTypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Type Variable {}", self.var)
    }
}

impl From<FreeTypeVariable> for EnvError {
    fn from(ftv: FreeTypeVariable) -> EnvError {
        EnvError::FreeTypeVariable(ftv)
    }
}

impl std::error::Error for FreeTypeVariable {}
