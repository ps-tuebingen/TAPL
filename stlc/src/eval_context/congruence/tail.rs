use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::TailList, types::Type};

#[derive(Debug, PartialEq, Eq)]
pub struct Tail {
    pub inner_type: Type,
    pub list: Box<EvalContext>,
}

impl Eval for Tail {
    fn eval(self) -> Result<Value, Error> {
        let val = self.list.eval()?;
        let ctx: EvalContext = TailList { list: val }.into();
        ctx.eval()
    }
}

impl From<Tail> for CongruenceRule {
    fn from(tail: Tail) -> CongruenceRule {
        CongruenceRule::Tail(tail)
    }
}

impl From<Tail> for EvalContext {
    fn from(tail: Tail) -> EvalContext {
        EvalContext::Congruence(tail.into())
    }
}

#[cfg(test)]
mod tail_tests {
    use super::{Eval, Tail};
    use crate::{
        eval_context::{computation::SuccPred, congruence::Cons1, Value},
        syntax::Nil,
        types::Type,
    };

    #[test]
    fn eval_tail() {
        let result = Tail {
            inner_type: Type::Nat,
            list: Box::new(
                Cons1 {
                    fst: Box::new(SuccPred { val: Value::Zero }.into()),
                    rst: Nil {
                        inner_type: Type::Nat,
                    }
                    .into(),
                    inner_type: Type::Nat,
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Nil {
            inner_type: Type::Nat,
        };
        assert_eq!(result, expected)
    }
}
