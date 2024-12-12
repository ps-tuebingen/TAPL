use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::Proj2Beta;

#[derive(Debug, PartialEq, Eq)]
pub struct Proj2 {
    pub term: Box<EvalContext>,
}

impl Eval for Proj2 {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        let ctx: EvalContext = Proj2Beta { pair: val }.into();
        ctx.eval()
    }
}

impl From<Proj2> for CongruenceRule {
    fn from(proj2: Proj2) -> CongruenceRule {
        CongruenceRule::Proj2(proj2)
    }
}

impl From<Proj2> for EvalContext {
    fn from(proj2: Proj2) -> EvalContext {
        EvalContext::Congruence(proj2.into())
    }
}

#[cfg(test)]
mod proj2_tests {
    use super::{Eval, Proj2};
    use crate::{
        eval_context::{computation::SuccPred, congruence::Pair1, Value},
        syntax::True,
    };

    #[test]
    fn eval_proj2() {
        let result = Proj2 {
            term: Box::new(
                Pair1 {
                    fst: Box::new(SuccPred { val: Value::Zero }.into()),
                    snd: True.into(),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
