use super::Subst;
use crate::{
    terms::syntax::{Term, Unit},
    Var,
};

impl Subst for Unit {
    type Target = Unit;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}
