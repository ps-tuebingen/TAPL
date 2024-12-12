use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::HeadList, types::Type};

#[derive(Debug, PartialEq, Eq)]
pub struct Head {
    pub inner_type: Type,
    pub list: Box<EvalContext>,
}

impl Eval for Head {
    fn eval(self) -> Result<Value, Error> {
        let val = self.list.eval()?;
        let ctx: EvalContext = HeadList { list: val }.into();
        ctx.eval()
    }
}

impl From<Head> for CongruenceRule {
    fn from(head: Head) -> CongruenceRule {
        CongruenceRule::Head(head)
    }
}

impl From<Head> for EvalContext {
    fn from(head: Head) -> EvalContext {
        EvalContext::Congruence(head.into())
    }
}

#[cfg(test)]
mod head_tests {
    use super::{Eval, Head};
    use crate::{
        eval_context::{computation::TailList, Value},
        types::Type,
    };

    #[test]
    fn eval_head() {
        let result = Head {
            inner_type: Type::Nat,
            list: Box::new(
                TailList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Cons {
                            fst: Box::new(Value::Succ(Box::new(Value::Zero))),
                            rst: Box::new(Value::Nil {
                                inner_type: Type::Nat,
                            }),
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
