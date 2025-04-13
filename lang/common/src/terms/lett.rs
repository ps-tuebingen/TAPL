use super::Term;
use crate::Var;

pub struct Let<T>
where
    T: Term,
{
    var: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Let<T> where T: Term {}
