use super::{CongruenceRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Pair2 {
    pub fst: Value,
    pub snd: Box<EvalContext>,
}

impl Eval for Pair2 {
    fn eval(self) -> Result<Value, Error> {
        let val2 = self.snd.eval()?;
        Ok(Value::Pair {
            fst: Box::new(self.fst),
            snd: Box::new(val2),
        })
    }
}

impl From<Pair2> for CongruenceRule {
    fn from(pair2: Pair2) -> CongruenceRule {
        CongruenceRule::Pair2(pair2)
    }
}

impl From<Pair2> for EvalContext {
    fn from(pair2: Pair2) -> EvalContext {
        EvalContext::Congruence(pair2.into())
    }
}

#[cfg(test)]
mod pair2_tests {
    use super::{Eval, Pair2};
    use crate::eval_context::{computation::IsZeroNum, Value};

    #[test]
    fn eval_pair2() {
        let result = Pair2 {
            fst: Value::Zero,
            snd: Box::new(IsZeroNum { num: Value::Zero }.into()),
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
