use super::{pair2::Pair2, CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct Pair1 {
    pub fst: Box<EvalContext>,
    pub snd: Term,
}

impl Eval for Pair1 {
    fn eval(self) -> Result<Value, Error> {
        let val = self.fst.eval()?;
        let inner_ctx: EvalContext = self.snd.to_context()?;
        let ctx: EvalContext = Pair2 {
            fst: val,
            snd: Box::new(inner_ctx),
        }
        .into();
        ctx.eval()
    }
}

impl From<Pair1> for CongruenceRule {
    fn from(pair1: Pair1) -> CongruenceRule {
        CongruenceRule::Pair1(pair1)
    }
}

impl From<Pair1> for EvalContext {
    fn from(pair1: Pair1) -> EvalContext {
        EvalContext::Congruence(pair1.into())
    }
}

#[cfg(test)]
mod pair1_tests {
    use super::{Eval, Pair1};
    use crate::{
        eval_context::{computation::SuccPred, Value},
        syntax::True,
    };

    #[test]
    fn eval_pair1() {
        let result = Pair1 {
            fst: Box::new(SuccPred { val: Value::Zero }.into()),
            snd: True.into(),
        }
        .eval()
        .unwrap();
        let expected = Value::Pair {
            fst: Box::new(Value::Zero),
            snd: Box::new(Value::True),
        };
        assert_eq!(result, expected)
    }
}
