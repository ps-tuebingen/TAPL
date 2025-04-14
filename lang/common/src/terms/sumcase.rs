use super::Term;
use crate::Var;
use std::fmt;

#[derive(Clone, Debug)]
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

impl<T> fmt::Display for SumCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ inl({}) => {} | inr({}) => {} }}",
            self.bound_term, self.left_var, self.left_term, self.right_var, self.right_term
        )
    }
}
