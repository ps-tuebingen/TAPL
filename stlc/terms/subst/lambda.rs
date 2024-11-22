use super::Subst;
use crate::{
    terms::syntax::{App, Lambda, Term},
    Var,
};

impl Subst for Lambda {
    type Target = Lambda;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        if var == self.var {
            self
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: self.body.subst(var, term),
            }
        }
    }
}

impl Subst for App {
    type Target = App;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        App {
            fun: self.fun.subst(var.clone(), term.clone()),
            arg: self.arg.subst(var, term),
        }
    }
}
