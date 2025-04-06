use crate::terms::{Term, Var};

pub mod bool;
pub mod fix;
pub mod lambda;
pub mod nat;
pub mod pack;
pub mod record;
pub mod term;

pub trait SubstTerm: Sized {
    fn subst(self, v: &Var, t: Term) -> Term;
}

impl SubstTerm for Var {
    fn subst(self, v: &Var, t: Term) -> Term {
        if self == *v {
            t
        } else {
            Term::Var(self)
        }
    }
}
