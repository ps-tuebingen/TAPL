use super::Term;
use crate::Var;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ListCase<T>
where
    T: Term,
{
    bound_term: Box<T>,
    nil_rhs: Box<T>,
    cons_fst: Var,
    cons_rst: Var,
    cons_rhs: Box<T>,
}

impl<T> Term for ListCase<T> where T: Term {}

impl<T> fmt::Display for ListCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ nil => {} | cons({},{}) => {} }}",
            self.bound_term, self.nil_rhs, self.cons_fst, self.cons_rst, self.cons_rhs
        )
    }
}
