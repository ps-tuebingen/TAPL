use super::{ComputationRule, Eval, EvalContext, Value};
use crate::eval::to_eval_err;
use common::errors::{Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct ProjBeta {
    pub tup: Value,
    pub ind: usize,
}

impl Eval for ProjBeta {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Tup(ref vals) = self.tup {
            vals.get(self.ind)
                .cloned()
                .ok_or(to_eval_err(ErrorKind::ValueMismatch {
                    expected: format!("Tuple with at least {} terms", self.ind),
                    found: self.tup.to_string(),
                }))
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: self.tup.to_string(),
                expected: "Tuple".to_owned(),
            }))
        }
    }
}

impl From<ProjBeta> for ComputationRule {
    fn from(proj: ProjBeta) -> ComputationRule {
        ComputationRule::ProjBeta(proj)
    }
}

impl From<ProjBeta> for EvalContext {
    fn from(proj: ProjBeta) -> EvalContext {
        EvalContext::Computation(proj.into())
    }
}

#[cfg(test)]
mod projbeta_tests {
    use super::{Eval, ProjBeta};
    use crate::{eval_context::Value, types::Type};

    #[test]
    fn eval_projbeta() {
        let result = ProjBeta {
            tup: Value::Tup(vec![
                Value::True,
                Value::False,
                Value::Nil {
                    inner_type: Type::Nat,
                },
            ]),
            ind: 1,
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
