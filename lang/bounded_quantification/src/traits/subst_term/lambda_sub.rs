use super::SubstTerm;
use crate::syntax::{LambdaSub, Term, TyApp, Var};

impl SubstTerm for LambdaSub {
    fn subst(self, v: &Var, t: Term) -> Term {
        if *v == self.var {
            self.into()
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: self.sup_ty,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl SubstTerm for TyApp {
    fn subst(self, v: &Var, t: Term) -> Term {
        TyApp {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}
