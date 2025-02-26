use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct IsZeroNum {
    pub num: Value,
}

impl Eval for IsZeroNum {
    fn eval(self) -> Result<Value, Error> {
        match self.num {
            Value::Zero => Ok(Value::True),
            Value::Pred(_) => Ok(Value::False),
            Value::Succ(_) => Ok(Value::False),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl From<IsZeroNum> for ComputationRule {
    fn from(iszeronum: IsZeroNum) -> ComputationRule {
        ComputationRule::IsZeroNum(iszeronum)
    }
}

impl From<IsZeroNum> for EvalContext {
    fn from(isz: IsZeroNum) -> EvalContext {
        EvalContext::Computation(isz.into())
    }
}

#[cfg(test)]
mod iszeronum_tests {
    use super::{Eval, IsZeroNum};
    use crate::eval_context::Value;

    #[test]
    fn eval_iszeronum() {
        let result = IsZeroNum { num: Value::Zero }.eval().unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
