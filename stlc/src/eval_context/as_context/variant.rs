use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::VariantCaseRhs, congruence, Value},
    syntax::{Variant, VariantCase},
};

impl AsContext for Variant {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(val) => Ok(EvalContext::Value(Value::Variant {
                label: self.label,
                ty: self.ty,
                val: Box::new(val),
            })),
            Err(_) => {
                let ctx = (*self.term).to_context()?;
                Ok(congruence::Variant {
                    term: Box::new(ctx),
                    label: self.label,
                    ty: self.ty,
                }
                .into())
            }
        }
    }
}

impl AsContext for VariantCase {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.bound_term).try_into() {
            Ok(val) => Ok(VariantCaseRhs {
                bound_val: val,
                patterns: self.cases,
            }
            .into()),
            Err(_) => {
                let bound_ctx = (*self.bound_term).to_context()?;
                Ok(congruence::VariantCase {
                    bound_term: Box::new(bound_ctx),
                    patterns: self.cases,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod variant_tests {
    use super::{AsContext, EvalContext, Variant, VariantCase};
    use crate::{
        eval_context::{
            computation::{IsZeroNum, VariantCaseRhs},
            congruence, Value,
        },
        syntax::{False, IsZero, True, VariantPattern, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    fn example_ty() -> Type {
        Type::Variant(HashMap::from([
            ("label".to_owned(), Type::Bool),
            ("other".to_owned(), Type::Nat),
        ]))
    }

    #[test]
    fn ctx_var_val() {
        let result = Variant {
            term: Box::new(True.into()),
            label: "label".to_owned(),
            ty: example_ty(),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Variant {
            label: "label".to_owned(),
            val: Box::new(Value::True),
            ty: example_ty(),
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_var_cong() {
        let result = Variant {
            label: "label".to_owned(),
            term: Box::new(
                IsZero {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            ty: example_ty(),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Variant {
            label: "label".to_owned(),
            term: Box::new(IsZeroNum { num: Value::Zero }.into()),
            ty: example_ty(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_case_beta() {
        let result = VariantCase {
            bound_term: Box::new(
                Variant {
                    label: "label".to_owned(),
                    term: Box::new(True.into()),
                    ty: example_ty(),
                }
                .into(),
            ),
            cases: vec![
                VariantPattern {
                    bound_var: "x".to_owned(),
                    label: "label".to_owned(),
                    rhs: Box::new(True.into()),
                },
                VariantPattern {
                    bound_var: "y".to_owned(),
                    label: "other".to_owned(),
                    rhs: Box::new(False.into()),
                },
            ],
        }
        .to_context()
        .unwrap();
        let expected = VariantCaseRhs {
            bound_val: Value::Variant {
                label: "label".to_owned(),
                ty: example_ty(),
                val: Box::new(Value::True),
            },
            patterns: vec![
                VariantPattern {
                    label: "label".to_owned(),
                    bound_var: "x".to_owned(),
                    rhs: Box::new(True.into()),
                },
                VariantPattern {
                    label: "other".to_owned(),
                    bound_var: "y".to_owned(),
                    rhs: Box::new(False.into()),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_case_cong() {
        let result = VariantCase {
            bound_term: Box::new(
                Variant {
                    label: "label".to_owned(),
                    term: Box::new(
                        IsZero {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                    ty: example_ty(),
                }
                .into(),
            ),
            cases: vec![
                VariantPattern {
                    label: "label".to_owned(),
                    bound_var: "x".to_owned(),
                    rhs: Box::new(True.into()),
                },
                VariantPattern {
                    label: "other".to_owned(),
                    bound_var: "y".to_owned(),
                    rhs: Box::new(False.into()),
                },
            ],
        }
        .to_context()
        .unwrap();
        let expected = congruence::VariantCase {
            bound_term: Box::new(
                congruence::Variant {
                    label: "label".to_owned(),
                    ty: example_ty(),
                    term: Box::new(IsZeroNum { num: Value::Zero }.into()),
                }
                .into(),
            ),
            patterns: vec![
                VariantPattern {
                    label: "label".to_owned(),
                    bound_var: "x".to_owned(),
                    rhs: Box::new(True.into()),
                },
                VariantPattern {
                    label: "other".to_owned(),
                    bound_var: "y".to_owned(),
                    rhs: Box::new(False.into()),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }
}
