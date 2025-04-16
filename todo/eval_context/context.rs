use super::{ComputationRule, CongruenceRule, Eval, Value};
use crate::eval::to_eval_err;
use common::{
    errors::{Error, ErrorKind},
    Var,
};

#[derive(Debug, PartialEq, Eq)]
pub enum EvalContext {
    Value(Value),
    Var(Var),
    Computation(ComputationRule),
    Congruence(CongruenceRule),
}

impl Eval for EvalContext {
    fn eval(self) -> Result<Value, Error> {
        match self {
            EvalContext::Value(val) => Ok(val),
            EvalContext::Var(v) => Err(to_eval_err(ErrorKind::FreeVariable(v))),
            EvalContext::Computation(rule) => rule.eval(),
            EvalContext::Congruence(rule) => rule.eval(),
        }
    }
}

impl From<Value> for EvalContext {
    fn from(val: Value) -> EvalContext {
        EvalContext::Value(val)
    }
}

impl From<Var> for EvalContext {
    fn from(v: Var) -> EvalContext {
        EvalContext::Var(v)
    }
}
