use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::IsZeroNum;

#[derive(Debug, PartialEq, Eq)]
pub struct IsZero {
    pub term: Box<EvalContext>,
}

impl Eval for IsZero {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        let ctx: EvalContext = IsZeroNum { num: val }.into();
        ctx.eval()
    }
}

impl From<IsZero> for CongruenceRule {
    fn from(isz: IsZero) -> CongruenceRule {
        CongruenceRule::IsZero(isz)
    }
}

impl From<IsZero> for EvalContext {
    fn from(isz: IsZero) -> EvalContext {
        EvalContext::Congruence(isz.into())
    }
}

#[cfg(test)]
mod iszero_tests {
    use super::{Eval, IsZero};
    use crate::eval_context::{computation::SuccPred, Value};

    #[test]
    fn eval_iszero() {
        let result = IsZero {
            term: Box::new(SuccPred { val: Value::Zero }.into()),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
