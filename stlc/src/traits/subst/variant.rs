use super::Subst;
use crate::{
    syntax::{Term, Variant, VariantCase, VariantPattern},
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
        .subst("x".to_owned(), "y".to_owned().into());
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
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = VariantCase {
            cases: vec![
                VariantPattern {
                    bound_var: "x".to_owned(),
                    label: "x".to_owned(),
                    rhs: Box::new("x".to_owned().into()),
                },
                VariantPattern {
                    bound_var: "y".to_owned(),
                    label: "y".to_owned(),
                    rhs: Box::new("y".to_owned().into()),
                },
            ],
            bound_term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
