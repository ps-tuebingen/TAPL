use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct SuccPred {
    pub val: Value,
}

impl Eval for SuccPred {
    fn eval(self) -> Result<Value, Error> {
        Ok(self.val)
    }
}

impl From<SuccPred> for ComputationRule {
    fn from(sp: SuccPred) -> ComputationRule {
        ComputationRule::SuccPred(sp)
    }
}

impl From<SuccPred> for EvalContext {
    fn from(sp: SuccPred) -> EvalContext {
        EvalContext::Computation(sp.into())
    }
}

#[cfg(test)]
mod succpred_tests {
    use super::{Eval, SuccPred};
    use crate::eval_context::Value;

    #[test]
    fn eval_succpred() {
        let result = SuccPred { val: Value::Zero }.eval().unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
