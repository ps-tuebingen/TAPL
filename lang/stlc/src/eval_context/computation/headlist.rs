use super::{ComputationRule, Eval, EvalContext, Value};
use crate::eval::to_eval_err;
use common::errors::{Error, ErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct HeadList {
    pub list: Value,
}

impl Eval for HeadList {
    fn eval(self) -> Result<Value, Error> {
        match self.list {
            Value::Nil { .. } => Err(to_eval_err(ErrorKind::TermMismatch {
                found: self.list.to_string(),
                expected: "Non-Empty List".to_owned(),
            })),
            Value::Cons { fst, .. } => Ok(*fst),
            val => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "List".to_owned(),
            })),
        }
    }
}

impl From<HeadList> for ComputationRule {
    fn from(hd: HeadList) -> ComputationRule {
        ComputationRule::HeadList(hd)
    }
}

impl From<HeadList> for EvalContext {
    fn from(hd: HeadList) -> EvalContext {
        EvalContext::Computation(hd.into())
    }
}

#[cfg(test)]
mod headlist_tests {
    use super::{Eval, HeadList};
    use crate::{eval_context::Value, types::Type};

    #[test]
    fn eval_headlist() {
        let result = HeadList {
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
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
