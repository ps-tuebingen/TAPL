use super::Subst;
use crate::{
    syntax::{Term, Variant, VariantCase, VariantPattern},
    traits::free_vars::{fresh_var, FreeVars},
    Var,
};

impl Subst for Variant {
    type Target = Variant;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Variant {
            label: self.label,
            term: self.term.subst(var, term),
            ty: self.ty,
        }
    }
}

impl Subst for VariantCase {
    type Target = VariantCase;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        let cases = self
            .cases
            .into_iter()
            .map(|case| case.subst(&var, term.clone()))
            .collect();
        VariantCase {
            bound_term: self.bound_term.subst(var, term),
            cases,
        }
    }
}

impl Subst for VariantPattern {
    type Target = VariantPattern;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        let mut free_v = self.free_vars();
        free_v.insert(self.bound_var.clone());
        free_v.insert(var.clone());

        let new_v = fresh_var(&free_v);
        let rhs_subst = self.rhs.subst(&self.bound_var, new_v.clone().into());

        VariantPattern {
            label: self.label,
            bound_var: new_v,
            rhs: rhs_subst.subst(var, term),
        }
    }
}

#[cfg(test)]
mod variant_tests {
    use super::{Subst, Variant, VariantCase, VariantPattern};
    use crate::types::Type;
    use std::collections::HashMap;

    #[test]
    fn subst_var() {
        let result = Variant {
            label: "x".to_owned(),
            term: Box::new("x".to_owned().into()),
            ty: Type::Variant(HashMap::from([("x".to_owned(), Type::Bool)])),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Variant {
            label: "x".to_owned(),
            term: Box::new("y".to_owned().into()),
            ty: Type::Variant(HashMap::from([("x".to_owned(), Type::Bool)])),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_case() {
        let result = VariantCase {
            cases: vec![
                VariantPattern {
                    bound_var: "x".to_owned(),
                    label: "x".to_owned(),
                    rhs: Box::new("x".to_owned().into()),
                },
                VariantPattern {
                    bound_var: "y".to_owned(),
                    label: "y".to_owned(),
                    rhs: Box::new("x".to_owned().into()),
                },
            ],
            bound_term: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = VariantCase {
            cases: vec![
                VariantPattern {
                    bound_var: "x0".to_owned(),
                    label: "x".to_owned(),
                    rhs: Box::new("x0".to_owned().into()),
                },
                VariantPattern {
                    bound_var: "x0".to_owned(),
                    label: "y".to_owned(),
                    rhs: Box::new("y".to_owned().into()),
                },
            ],
            bound_term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
