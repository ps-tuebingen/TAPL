use super::{ComputationRule, Eval, EvalContext, Value};
use crate::{
    eval::to_eval_err,
    eval_context::AsContext,
    syntax::{Fix, Lambda},
    traits::subst::Subst,
};
use common::errors::{Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct FixBeta {
    pub lam: Value,
}

impl Eval for FixBeta {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Lambda { var, annot, body } = self.lam {
            let t = body.clone().subst(
                &var,
                Fix {
                    term: Box::new(
                        Lambda {
                            var: var.clone(),
                            annot,
                            body: Box::new(body),
                        }
                        .into(),
                    ),
                }
                .into(),
            );
            let ctx: EvalContext = t.to_context()?;
            ctx.eval()
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: self.lam.to_string(),
                expected: "Function".to_owned(),
            }))
        }
    }
}

impl From<FixBeta> for ComputationRule {
    fn from(fix: FixBeta) -> ComputationRule {
        ComputationRule::FixBeta(fix)
    }
}

impl From<FixBeta> for EvalContext {
    fn from(fix: FixBeta) -> EvalContext {
        EvalContext::Computation(fix.into())
    }
}

#[cfg(test)]
mod fixbeta_tests {
    use super::{Eval, FixBeta};
    use crate::{eval_context::Value, syntax::Zero, types::Type};

    #[test]
    fn eval_fixbeta() {
        let result = FixBeta {
            lam: Value::Lambda {
                var: "x".to_owned(),
                annot: Type::Nat,
                body: Zero.into(),
            },
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
