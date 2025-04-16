use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::Proj1Beta;

#[derive(Debug, PartialEq, Eq)]
pub struct Proj1 {
    pub term: Box<EvalContext>,
}

impl Eval for Proj1 {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        let ctx: EvalContext = Proj1Beta { pair: val }.into();
        ctx.eval()
    }
}

impl From<Proj1> for CongruenceRule {
    fn from(proj: Proj1) -> CongruenceRule {
        CongruenceRule::Proj1(proj)
    }
}

impl From<Proj1> for EvalContext {
    fn from(proj1: Proj1) -> EvalContext {
        EvalContext::Congruence(proj1.into())
    }
}

#[cfg(test)]
mod proj1_tests {
    use super::{Eval, Proj1};
    use crate::eval_context::{computation::IsZeroNum, congruence::Pair2, Value};

    #[test]
    fn eval_proj1() {
        let result = Proj1 {
            term: Box::new(
                Pair2 {
                    fst: Value::Zero,
                    snd: Box::new(IsZeroNum { num: Value::Zero }.into()),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
