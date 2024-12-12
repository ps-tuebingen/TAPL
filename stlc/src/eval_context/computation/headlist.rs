use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct HeadList {
    pub list: Value,
}

impl Eval for HeadList {
    fn eval(self) -> Result<Value, Error> {
        match self.list {
            Value::Nil { .. } => Err(Error::HeadOfEmptyList),
            Value::Cons { fst, .. } => Ok(*fst),
            val => Err(Error::BadValue { val }),
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
