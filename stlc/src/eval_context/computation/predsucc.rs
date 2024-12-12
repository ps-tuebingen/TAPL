use super::{ComputationRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct PredSucc {
    pub val: Value,
}

impl Eval for PredSucc {
    fn eval(self) -> Result<Value, Error> {
        Ok(self.val)
    }
}

impl From<PredSucc> for ComputationRule {
    fn from(ps: PredSucc) -> ComputationRule {
        ComputationRule::PredSucc(ps)
    }
}

impl From<PredSucc> for EvalContext {
    fn from(ps: PredSucc) -> EvalContext {
        EvalContext::Computation(ps.into())
    }
}

#[cfg(test)]
mod predsucc_tests {
    use super::{Eval, PredSucc};
    use crate::eval_context::Value;

    #[test]
    fn eval_predsucc() {
        let result = PredSucc { val: Value::Zero }.eval().unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
