use super::Term;
use crate::Var;

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
