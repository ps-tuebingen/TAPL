use super::Subst;
use crate::{
    terms::syntax::{Pair, Proj1, Proj2, Term},
    Var,
};

impl Subst for Pair {
    type Target = Pair;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Pair {
            fst: self.fst.subst(var.clone(), term.clone()),
            snd: self.snd.subst(var, term),
        }
    }
}
impl Subst for Proj1 {
    type Target = Proj1;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Proj1 {
            pair: self.pair.subst(var, term),
        }
    }
}
impl Subst for Proj2 {
    type Target = Proj2;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Proj2 {
            pair: self.pair.subst(var, term),
        }
    }
}
