use super::SubstTerm;
use crate::terms::{Pack, Term, Unpack, Var};

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
        {
            let in_subst = self.in_term.subst(v, t.clone());
            if self.bound_var == *v {
                Unpack {
                    ty_var: self.ty_var,
                    bound_var: self.bound_var,
                    bound_term: self.bound_term,
                    in_term: Box::new(in_subst),
                }
                .into()
            } else {
                Unpack {
                    ty_var: self.ty_var,
                    bound_var: self.bound_var,
                    bound_term: Box::new(self.bound_term.subst(v, t)),
                    in_term: Box::new(in_subst),
                }
                .into()
            }
        }
    }
}
