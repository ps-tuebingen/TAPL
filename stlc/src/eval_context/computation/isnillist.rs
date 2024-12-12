use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct IsNilList {
    pub list: Value,
}

impl Eval for IsNilList {
    fn eval(self) -> Result<Value, Error> {
        match self.list {
            Value::Nil { .. } => Ok(Value::True),
            Value::Cons { .. } => Ok(Value::False),
            val => Err(Error::BadValue { val }),
        }
    }
}
impl From<IsNilList> for ComputationRule {
    fn from(isnil: IsNilList) -> ComputationRule {
        ComputationRule::IsNilList(isnil)
    }
}

impl From<IsNilList> for EvalContext {
    fn from(isnil: IsNilList) -> EvalContext {
        EvalContext::Computation(isnil.into())
    }
}

#[cfg(test)]
mod isnillist_tests {
    use super::{Eval, IsNilList};
    use crate::{eval_context::Value, types::Type};

    #[test]
    fn eval_isnillist() {
        let result = IsNilList {
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
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
