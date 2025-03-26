use super::SubstTerm;
use crate::syntax::{Const, Pred, Succ, Term, Var};

impl SubstTerm for Const {
    fn subst(self, _: &Var, _: Term) -> Term {
        self.into()
    }
}

impl SubstTerm for Succ {
    fn subst(self, v: &Var, t: Term) -> Term {
        let inner_subst = self.term.subst(v, t);
        Succ {
            term: Box::new(inner_subst),
        }
        .into()
    }
}

impl SubstTerm for Pred {
    fn subst(self, v: &Var, t: Term) -> Term {
        let inner_subst = self.term.subst(v, t);
        Pred {
            term: Box::new(inner_subst),
        }
        .into()
    }
}
