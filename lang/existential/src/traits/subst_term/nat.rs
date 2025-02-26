use super::SubstTerm;
use crate::terms::{IsZero, Pred, Succ, Term, Var, Zero};

impl SubstTerm for Zero {
    fn subst(self, _: &Var, _: Term) -> Term {
        Zero.into()
    }
}

impl SubstTerm for Succ {
    fn subst(self, v: &Var, t: Term) -> Term {
        Succ(Box::new(self.0.subst(v, t))).into()
    }
}

impl SubstTerm for Pred {
    fn subst(self, v: &Var, t: Term) -> Term {
        Pred(Box::new(self.0.subst(v, t))).into()
    }
}

impl SubstTerm for IsZero {
    fn subst(self, v: &Var, t: Term) -> Term {
        IsZero(Box::new(self.0.subst(v, t))).into()
    }
}
