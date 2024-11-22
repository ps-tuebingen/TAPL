use super::Subst;
use crate::{
    terms::{
        free_vars::FreeVars,
        syntax::{Let, Term},
    },
    Var,
};

impl Subst for Let {
    type Target = Let;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        let in_term = if self.var == var || self.free_vars().contains(&var) {
            self.in_term
        } else {
            self.in_term.subst(var.clone(), term.clone())
        };
        Let {
            var: self.var,
            bound_term: self.bound_term.subst(var, term),
            in_term,
        }
    }
}
