use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct TailList {
    pub list: Value,
}

impl Eval for TailList {
    fn eval(self) -> Result<Value, Error> {
        match self.list {
            Value::Nil { .. } => Err(Error::TailOfEmptyList),
            Value::Cons { rst, .. } => Ok(*rst),
            val => Err(Error::BadValue { val }),
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
