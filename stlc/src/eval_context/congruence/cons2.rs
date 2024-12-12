use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::types::Type;

#[derive(Debug, PartialEq, Eq)]
pub struct Cons2 {
    pub inner_type: Type,
    pub fst: Value,
    pub rst: Box<EvalContext>,
}

impl Eval for Cons2 {
    fn eval(self) -> Result<Value, Error> {
        let rst_val = self.rst.eval()?;
        Ok(Value::Cons {
            fst: Box::new(self.fst),
            rst: Box::new(rst_val),
            inner_type: self.inner_type,
        })
    }
}

impl From<Cons2> for CongruenceRule {
    fn from(right: Cons2) -> CongruenceRule {
        CongruenceRule::Cons2(right)
    }
}

impl From<Cons2> for EvalContext {
    fn from(right: Cons2) -> EvalContext {
        EvalContext::Congruence(right.into())
    }
}

#[cfg(test)]
mod cons2_tests {
    use super::{Cons2, Eval, Value};
    use crate::{eval_context::computation::TailList, types::Type};

    #[test]
    fn eval_cons2() {
        let result = Cons2 {
            fst: Value::Zero,
            rst: Box::new(
                TailList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Nat,
                        }),
                        inner_type: Type::Nat,
                    }
                    .into(),
                }
                .into(),
            ),
            inner_type: Type::Nat,
        }
        .eval()
        .unwrap();
        let expected = Value::Cons {
            inner_type: Type::Nat,
            fst: Box::new(Value::Zero),
            rst: Box::new(Value::Nil {
                inner_type: Type::Nat,
            }),
        };
        assert_eq!(result, expected)
    }
}
