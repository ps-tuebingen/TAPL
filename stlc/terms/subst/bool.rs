use super::Subst;
use crate::{
    terms::syntax::{False, If, Term, True},
    Var,
};

impl Subst for True {
    type Target = True;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for False {
    type Target = False;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for If {
    type Target = If;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        If {
            ifc: self.ifc.subst(var.clone(), term.clone()),
            thenc: self.thenc.subst(var.clone(), term.clone()),
            elsec: self.elsec.subst(var, term),
        }
    }
}
