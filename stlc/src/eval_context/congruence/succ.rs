use super::{CongruenceRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Succ {
    pub term: Box<EvalContext>,
}

impl Eval for Succ {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Succ(Box::new(val)))
    }
}

impl From<Succ> for CongruenceRule {
    fn from(s: Succ) -> CongruenceRule {
        CongruenceRule::Succ(s)
    }
}

impl From<Succ> for EvalContext {
    fn from(s: Succ) -> EvalContext {
        EvalContext::Congruence(s.into())
    }
}

#[cfg(test)]
mod succ_tests {
    use super::{Eval, Succ};
    use crate::{
        eval_context::{computation::HeadList, Value},
        types::Type,
    };

    #[test]
    fn eval_succ() {
        let result = Succ {
            term: Box::new(
                HeadList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Nat,
                        }),
                        inner_type: Type::Nat,
                    },
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Succ(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }
}
