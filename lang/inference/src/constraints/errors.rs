use crate::{types::Type, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    VariableNotFound { var: Var },
    CannotUnify { ty1: Type, ty2: Type },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VariableNotFound { var } => {
                write!(f, "Could not find variable {var} in typing environment")
            }
            Error::CannotUnify { ty1, ty2 } => write!(f, "Cannot unify types {ty1} and {ty2}"),
        }
    }
}

impl std::error::Error for Error {}
