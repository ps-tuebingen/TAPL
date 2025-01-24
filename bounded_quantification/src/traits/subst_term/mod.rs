use crate::syntax::{Term, Var};

pub mod lambda;
pub mod lambda_sub;
pub mod pack;
pub mod term;

pub trait SubstTerm {
    fn subst(self, v: &Var, t: Term) -> Term;
}
