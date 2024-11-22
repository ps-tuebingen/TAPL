use super::Subst;
use crate::{
    terms::syntax::{Fix, Term},
    Var,
};

impl Subst for Fix {
    type Target = Fix;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Fix {
            term: self.term.subst(var, term),
        }
    }
}
