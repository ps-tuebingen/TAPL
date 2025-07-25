use crate::Var;
use std::fmt;

#[derive(Debug)]
pub struct FreeVariable {
    var: Var,
}

impl FreeVariable {
    pub fn new(var: &Var) -> FreeVariable {
        FreeVariable { var: var.clone() }
    }
}

impl fmt::Display for FreeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Variable {}", self.var)
    }
}

impl std::error::Error for FreeVariable {}
