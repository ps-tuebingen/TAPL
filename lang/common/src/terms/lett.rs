use super::Term;
use crate::Var;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Let<T>
where
    T: Term,
{
    var: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Let<T> where T: Term {}

impl<T> fmt::Display for Let<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
