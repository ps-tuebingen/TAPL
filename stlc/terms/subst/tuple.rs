use super::Subst;
use crate::{
    terms::syntax::{Proj, Term, Tup},
    Var,
};

impl Subst for Tup {
    type Target = Tup;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Tup {
            terms: self
                .terms
                .into_iter()
                .map(|t| t.subst(var.clone(), term.clone()))
                .collect(),
        }
    }
}

impl Subst for Proj {
    type Target = Proj;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Proj {
            tup: self.tup.subst(var, term),
            ind: self.ind,
        }
    }
}
