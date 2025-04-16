use super::{ComputationRule, Eval, EvalContext, Value};
use crate::eval::to_eval_err;
use common::errors::{Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct Proj2Beta {
    pub pair: Value,
}

impl Eval for Proj2Beta {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Pair { snd, .. } = self.pair {
            Ok(*snd)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: self.pair.to_string(),
                expected: "Pair".to_owned(),
            }))
        }
    }
}
impl From<Proj2Beta> for ComputationRule {
    fn from(proj: Proj2Beta) -> ComputationRule {
        ComputationRule::Proj2Beta(proj)
    }
}

impl From<Proj2Beta> for EvalContext {
    fn from(proj: Proj2Beta) -> EvalContext {
        EvalContext::Computation(proj.into())
    }
}

#[cfg(test)]
mod proj2beta_tests {
    use super::{Eval, Proj2Beta};
    use crate::eval_context::Value;

    #[test]
    fn eval_proj2beta() {
        let result = Proj2Beta {
            pair: Value::Pair {
                fst: Box::new(Value::Zero),
                snd: Box::new(Value::True),
            },
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
