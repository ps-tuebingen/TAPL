use super::{ComputationRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::VariantPattern, traits::subst::Subst};

#[derive(Debug, PartialEq, Eq)]
pub struct VariantCaseRhs {
    pub bound_val: Value,
    pub patterns: Vec<VariantPattern>,
}

impl Eval for VariantCaseRhs {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Variant { label, ty: _, val } = self.bound_val {
            let matching = self
                .patterns
                .into_iter()
                .find(|pt| pt.label == label)
                .ok_or(Error::MissingPattern { label })?;
            let t = matching.rhs.subst(&matching.bound_var, (*val).into());
            let ctx: EvalContext = (*t).to_context()?;
            ctx.eval()
        } else {
            Err(Error::BadValue {
                val: self.bound_val,
            })
        }
    }
}

impl From<VariantCaseRhs> for ComputationRule {
    fn from(case: VariantCaseRhs) -> ComputationRule {
        ComputationRule::VariantCaseRhs(case)
    }
}

impl From<VariantCaseRhs> for EvalContext {
    fn from(case: VariantCaseRhs) -> EvalContext {
        EvalContext::Computation(case.into())
    }
}

#[cfg(test)]
mod variantcaserhs_tests {
    use super::{Eval, VariantCaseRhs};
    use crate::{
        eval_context::Value,
        syntax::{IsZero, VariantPattern},
        types::Type,
    };
    use std::collections::HashMap;

    fn example_ty() -> Type {
        Type::Variant(HashMap::from([
            ("label".to_owned(), Type::Nat),
            ("other".to_owned(), Type::Bool),
        ]))
    }

    #[test]
    fn eval_variantcaserhs() {
        let result = VariantCaseRhs {
            bound_val: Value::Variant {
                label: "label".to_owned(),
                ty: example_ty(),
                val: Box::new(Value::Zero),
            },
            patterns: vec![
                VariantPattern {
                    bound_var: "x".to_owned(),
                    label: "label".to_owned(),
                    rhs: Box::new(
                        IsZero {
                            term: Box::new("x".to_owned().into()),
                        }
                        .into(),
                    ),
                },
                VariantPattern {
                    bound_var: "y".to_owned(),
                    label: "other".to_owned(),
                    rhs: Box::new("y".to_owned().into()),
                },
            ],
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
