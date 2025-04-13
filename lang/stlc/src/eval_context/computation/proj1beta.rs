use super::{ComputationRule, Eval, EvalContext, Value};
use crate::eval::to_eval_err;
use common::errors::{Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct Proj1Beta {
    pub pair: Value,
}

impl Eval for Proj1Beta {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Pair { fst, .. } = self.pair {
            Ok(*fst)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: self.pair.to_string(),
                expected: "Pair".to_owned(),
            }))
        }
    }
}

impl From<Proj1Beta> for ComputationRule {
    fn from(proj: Proj1Beta) -> ComputationRule {
        ComputationRule::Proj1Beta(proj)
    }
}

impl From<Proj1Beta> for EvalContext {
    fn from(proj: Proj1Beta) -> EvalContext {
        EvalContext::Computation(proj.into())
    }
}

#[cfg(test)]
mod proj1beta_tests {
    use super::{Eval, Proj1Beta};
    use crate::eval_context::Value;

    #[test]
    fn eval_proj1beta() {
        let result = Proj1Beta {
            pair: Value::Pair {
                fst: Box::new(Value::Zero),
                snd: Box::new(Value::True),
            },
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
