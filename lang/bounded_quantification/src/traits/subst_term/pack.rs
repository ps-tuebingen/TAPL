use super::SubstTerm;
use crate::syntax::{Pack, Term, Unpack, Var};

impl SubstTerm for Pack {
    fn subst(self, v: &Var, t: Term) -> Term {
        Pack {
            inner_ty: self.inner_ty,
            term: Box::new(self.term.subst(v, t)),
            outer_ty: self.outer_ty,
        }
        .into()
    }
}

impl SubstTerm for Unpack {
    fn subst(self, v: &Var, t: Term) -> Term {
        let bound_subst = self.bound_term.subst(v, t.clone());

        if *v == self.bound_var {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_var: self.ty_var,
                bound_var: self.bound_var,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}
