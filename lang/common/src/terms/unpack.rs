use super::Term;
use crate::{TypeVar, Var};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Unpack<T>
where
    T: Term,
{
    ty_name: TypeVar,
    term_name: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Unpack<T> where T: Term {}

impl<T> fmt::Display for Unpack<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_name, self.term_name, self.bound_term, self.in_term
        )
    }
}
