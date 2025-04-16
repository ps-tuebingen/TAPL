use super::{CongruenceRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Something {
    pub term: Box<EvalContext>,
}

impl Eval for Something {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Something(Box::new(val)))
    }
}

impl From<Something> for CongruenceRule {
    fn from(some: Something) -> CongruenceRule {
        CongruenceRule::Something(some)
    }
}

impl From<Something> for EvalContext {
    fn from(some: Something) -> EvalContext {
        EvalContext::Congruence(some.into())
    }
}

#[cfg(test)]
mod something_tests {
    use super::{Eval, Something};
    use crate::eval_context::{computation::IsZeroNum, Value};

    #[test]
    fn eval_something() {
        let result = Something {
            term: Box::new(IsZeroNum { num: Value::Zero }.into()),
        }
        .eval()
        .unwrap();
        let expected = Value::Something(Box::new(Value::True));
        assert_eq!(result, expected)
    }
}
