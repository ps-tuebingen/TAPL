use crate::syntax::{Term, Var};

pub mod lambda;
pub mod lambda_sub;
pub mod nat;
pub mod pack;
pub mod record;
pub mod term;

pub trait SubstTerm {
    fn subst(self, v: &Var, t: Term) -> Term;
}
