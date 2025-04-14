use super::Term;
use crate::TypeVar;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TyLambda<T>
where
    T: Term,
{
    var: TypeVar,
    term: Box<T>,
}

impl<T> Term for TyLambda<T> where T: Term {}

impl<T> fmt::Display for TyLambda<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.term)
    }
}
