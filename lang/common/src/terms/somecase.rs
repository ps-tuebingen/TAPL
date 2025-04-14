use super::Term;
use crate::Var;
use std::fmt;

#[derive(Clone, Debug)]
pub struct SomeCase<T>
where
    T: Term,
{
    bound_term: Box<T>,
    none_term: Box<T>,
    some_var: Var,
    some_term: Box<T>,
}

impl<T> Term for SomeCase<T> where T: Term {}

impl<T> fmt::Display for SomeCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ nothing => {} | something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}
