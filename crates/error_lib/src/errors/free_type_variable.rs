use crate::TypeVar;
use std::fmt;

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

impl std::error::Error for FreeTypeVariable {}
