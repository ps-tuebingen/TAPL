use super::SubstTerm;
use crate::syntax::{App, Lambda, Term, Var};

impl SubstTerm for Lambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        if *v == self.var {
            self.into()
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl SubstTerm for App {
    fn subst(self, v: &Var, t: Term) -> Term {
        App {
            fun: Box::new(self.fun.subst(v, t.clone())),
            arg: Box::new(self.arg.subst(v, t)),
        }
        .into()
    }
}
