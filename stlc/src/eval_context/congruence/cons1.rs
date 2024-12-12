use super::{cons2::Cons2, CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term, types::Type};

#[derive(Debug, PartialEq, Eq)]
pub struct Cons1 {
    pub inner_type: Type,
    pub fst: Box<EvalContext>,
    pub rst: Term,
}

impl Eval for Cons1 {
    fn eval(self) -> Result<Value, Error> {
        let fst_val = self.fst.eval()?;
        let inner: EvalContext = self.rst.to_context()?;
        let ctx: EvalContext = Cons2 {
            fst: fst_val,
            rst: Box::new(inner),
            inner_type: self.inner_type,
        }
        .into();
        ctx.eval()
    }
}

impl From<Cons1> for CongruenceRule {
    fn from(cons1: Cons1) -> CongruenceRule {
        CongruenceRule::Cons1(cons1)
    }
}

impl From<Cons1> for EvalContext {
    fn from(cons1: Cons1) -> EvalContext {
        EvalContext::Congruence(cons1.into())
    }
}

#[cfg(test)]
mod cons1_tests {
    use super::{Cons1, Eval};
    use crate::{
        eval_context::{computation::PredSucc, Value},
        syntax::Nil,
        types::Type,
    };

    #[test]
    fn eval_cons1() {
        let result = Cons1 {
            inner_type: Type::Nat,
            fst: Box::new(PredSucc { val: Value::Zero }.into()),
            rst: Nil {
                inner_type: Type::Nat,
            }
            .into(),
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
