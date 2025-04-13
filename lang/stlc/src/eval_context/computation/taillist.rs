use super::{ComputationRule, Eval, EvalContext, Value};
use crate::eval::to_eval_err;
use common::errors::{Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct TailList {
    pub list: Value,
}

impl Eval for TailList {
    fn eval(self) -> Result<Value, Error> {
        match self.list {
            Value::Nil { .. } => Err(to_eval_err(ErrorKind::TermMismatch {
                found: self.list.to_string(),
                expected: "Non-Empty List".to_owned(),
            })),
            Value::Cons { rst, .. } => Ok(*rst),
            val => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "List".to_owned(),
            })),
        }
    }
}

impl From<TailList> for ComputationRule {
    fn from(tl: TailList) -> ComputationRule {
        ComputationRule::TailList(tl)
    }
}

impl From<TailList> for EvalContext {
    fn from(tl: TailList) -> EvalContext {
        EvalContext::Computation(tl.into())
    }
}

#[cfg(test)]
mod taillist_tests {
    use super::{Eval, TailList};
    use crate::{eval_context::Value, types::Type};

    #[test]
    fn eval_taillist() {
        let result = TailList {
            list: Value::Cons {
                fst: Box::new(Value::Zero),
                rst: Box::new(Value::Nil {
                    inner_type: Type::Nat,
                }),
                inner_type: Type::Nat,
            },
        }
        .eval()
        .unwrap();
        let expected = Value::Nil {
            inner_type: Type::Nat,
        };
        assert_eq!(result, expected)
    }
}
