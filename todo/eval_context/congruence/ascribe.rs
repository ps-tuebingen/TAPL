use super::{CongruenceRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Ascribe {
    pub term: Box<EvalContext>,
}

impl Eval for Ascribe {
    fn eval(self) -> Result<Value, Error> {
        self.term.eval()
    }
}

impl From<Ascribe> for CongruenceRule {
    fn from(asc: Ascribe) -> CongruenceRule {
        CongruenceRule::Ascribe(asc)
    }
}

impl From<Ascribe> for EvalContext {
    fn from(asc: Ascribe) -> EvalContext {
        EvalContext::Congruence(asc.into())
    }
}

#[cfg(test)]
mod ascribe_tests {
    use super::{Ascribe, Eval, EvalContext};
    use crate::eval_context::Value;

    #[test]
    fn eval_asc() {
        let result = Ascribe {
            term: Box::new(EvalContext::Value(Value::True)),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
