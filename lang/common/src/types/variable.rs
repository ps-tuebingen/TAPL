use super::Type;
use crate::TypeVar;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TypeVariable {
    v: TypeVar,
}

impl Type for TypeVariable {}

impl fmt::Display for TypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
