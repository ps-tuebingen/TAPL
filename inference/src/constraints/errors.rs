use std::fmt;
use crate::Var;

#[derive(Debug)]
pub enum Error{
    VariableNotFound{var:Var}
}

impl fmt::Display for Error{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        match self{
            Error::VariableNotFound{var} => write!(f,"Could not find variable {var} in typing environment")
        }
    }
}

impl std::error::Error for Error{}
