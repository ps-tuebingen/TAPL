use super::Term;
use crate::{types::Type, Var};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Left {
    pub left_term: Box<Term>,
    pub right_ty: Type,
}

#[derive(Debug, Clone)]
pub struct Right {
    pub right_term: Box<Term>,
    pub left_ty: Type,
}

#[derive(Debug, Clone)]
pub struct SumCase {
    pub bound_term: Box<Term>,
    pub left_var: Var,
    pub left_term: Box<Term>,
    pub right_var: Var,
    pub right_term: Box<Term>,
}

impl From<Left> for Term {
    fn from(left: Left) -> Term {
        Term::Left(left)
    }
}

impl From<Right> for Term {
    fn from(right: Right) -> Term {
        Term::Right(right)
    }
}

impl From<SumCase> for Term {
    fn from(sum: SumCase) -> Term {
        Term::SumCase(sum)
    }
}

impl fmt::Display for Left {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({})", self.left_term)
    }
}

impl fmt::Display for Right {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({})", self.right_term)
    }
}

impl fmt::Display for SumCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of inl({}) => {} | inr({}) => {}",
            self.bound_term, self.left_var, self.left_term, self.right_var, self.right_term
        )
    }
}
