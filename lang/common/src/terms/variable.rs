use super::Term;
use crate::Var;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Variable {
    var: Var,
}

impl Term for Variable {}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
