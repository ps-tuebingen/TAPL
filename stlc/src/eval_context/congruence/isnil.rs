use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::computation::IsNilList, types::Type};

#[derive(Debug, PartialEq, Eq)]
pub struct IsNil {
    pub inner_type: Type,
    pub list: Box<EvalContext>,
}

impl Eval for IsNil {
    fn eval(self) -> Result<Value, Error> {
        let val = self.list.eval()?;
        let ctx: EvalContext = IsNilList { list: val }.into();
        ctx.eval()
    }
}

impl From<IsNil> for CongruenceRule {
    fn from(isnil: IsNil) -> CongruenceRule {
        CongruenceRule::IsNil(isnil)
    }
}

impl From<IsNil> for EvalContext {
    fn from(isnil: IsNil) -> EvalContext {
        EvalContext::Congruence(isnil.into())
    }
}

#[cfg(test)]
mod isnil_tests {
    use super::{Eval, IsNil};
    use crate::{
        eval_context::{computation::TailList, Value},
        types::Type,
    };

    #[test]
    fn eval_isnil() {
        let result = IsNil {
            inner_type: Type::Nat,
            list: Box::new(
                TailList {
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
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
