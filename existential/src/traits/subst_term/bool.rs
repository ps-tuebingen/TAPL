use super::SubstTerm;
use crate::terms::{False, If, Term, True, Var};

impl SubstTerm for True {
    fn subst(self, _: &Var, _: Term) -> Term {
        self.into()
    }
}

impl SubstTerm for False {
    fn subst(self, _: &Var, _: Term) -> Term {
        self.into()
    }
}

impl SubstTerm for If {
    fn subst(self, v: &Var, t: Term) -> Term {
        If {
            ifc: Box::new(self.ifc.subst(v, t.clone())),
            thenc: Box::new(self.thenc.subst(v, t.clone())),
            elsec: Box::new(self.elsec.subst(v, t)),
        }
        .into()
    }
}
