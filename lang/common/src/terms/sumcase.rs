use super::Term;
use crate::Var;

pub struct SumCase<T>
where
    T: Term,
{
    pub bound_term: Box<T>,
    pub left_var: Var,
    pub left_term: Box<T>,
    pub right_var: Var,
    pub right_term: Box<T>,
}

impl<T> Term for SumCase<T> where T: Term {}
