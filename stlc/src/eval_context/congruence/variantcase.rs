use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::VariantCaseRhs, syntax::VariantPattern};

#[derive(Debug, PartialEq, Eq)]
pub struct VariantCase {
    pub bound_term: Box<EvalContext>,
    pub patterns: Vec<VariantPattern>,
}

impl Eval for VariantCase {
    fn eval(self) -> Result<Value, Error> {
        let val = self.bound_term.eval()?;
        let ctx: EvalContext = VariantCaseRhs {
            bound_val: val,
            patterns: self.patterns,
        }
        .into();
        ctx.eval()
    }
}

impl From<VariantCase> for CongruenceRule {
    fn from(case: VariantCase) -> CongruenceRule {
        CongruenceRule::VariantCase(case)
    }
}

impl From<VariantCase> for EvalContext {
    fn from(case: VariantCase) -> EvalContext {
        EvalContext::Congruence(case.into())
    }
}

#[cfg(test)]
mod variantcase_tests {
    use super::{Eval, VariantCase};
    use crate::{
        eval_context::{computation::SuccPred, congruence::Variant, Value},
        syntax::{False, IsZero, VariantPattern},
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
    fn eval_case() {
        let result = VariantCase {
            bound_term: Box::new(
                Variant {
                    label: "label".to_owned(),
                    term: Box::new(SuccPred { val: Value::Zero }.into()),
                    ty: example_ty(),
                }
                .into(),
            ),
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
                    rhs: Box::new(False.into()),
                },
            ],
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
