use super::{CongruenceRule, Error, Eval, EvalContext, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Pred {
    pub term: Box<EvalContext>,
}
impl Eval for Pred {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Pred(Box::new(val)))
    }
}
impl From<Pred> for CongruenceRule {
    fn from(p: Pred) -> CongruenceRule {
        CongruenceRule::Pred(p)
    }
}

impl From<Pred> for EvalContext {
    fn from(p: Pred) -> EvalContext {
        EvalContext::Congruence(p.into())
    }
}

#[cfg(test)]
mod pred_tests {
    use super::{Eval, Pred};
    use crate::{
        eval_context::{computation::HeadList, Value},
        types::Type,
    };

    #[test]
    fn eval_pred() {
        let result = Pred {
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
        let expected = Value::Pred(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }
}
