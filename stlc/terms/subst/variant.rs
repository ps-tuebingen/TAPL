use super::Subst;
use crate::{
    terms::syntax::{Term, Variant, VariantCase, VariantPattern},
    Var,
};

impl Subst for Variant {
    type Target = Variant;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Variant {
            label: self.label,
            term: self.term.subst(var, term),
            ty: self.ty,
        }
    }
}

impl Subst for VariantCase {
    type Target = VariantCase;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        let cases = self
            .cases
            .into_iter()
            .map(|case| case.subst(var.clone(), term.clone()))
            .collect();
        VariantCase {
            bound_term: self.bound_term.subst(var, term),
            cases,
        }
    }
}

impl Subst for VariantPattern {
    type Target = VariantPattern;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        let rhs = if var == self.bound_var {
            self.rhs
        } else {
            self.rhs.subst(var, term)
        };
        VariantPattern {
            label: self.label,
            bound_var: self.bound_var,
            rhs,
        }
    }
}
