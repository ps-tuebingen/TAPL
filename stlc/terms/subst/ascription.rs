use super::Subst;
use crate::{
    terms::syntax::{Ascribe, Term},
    Var,
};

impl Subst for Ascribe {
    type Target = Ascribe;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Ascribe {
            ty: self.ty,
            term: self.term.subst(var, term),
        }
    }
}
