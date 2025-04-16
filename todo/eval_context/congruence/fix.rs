use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::FixBeta;

#[derive(Debug, PartialEq, Eq)]
pub struct Fix {
    pub term: Box<EvalContext>,
}

impl Eval for Fix {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        let ctx: EvalContext = FixBeta { lam: val }.into();
        ctx.eval()
    }
}

impl From<Fix> for CongruenceRule {
    fn from(fix: Fix) -> CongruenceRule {
        CongruenceRule::Fix(fix)
    }
}

impl From<Fix> for EvalContext {
    fn from(fix: Fix) -> EvalContext {
        EvalContext::Congruence(fix.into())
    }
}

#[cfg(test)]
mod fix_tests {
    use super::{Eval, EvalContext, Fix};
    use crate::{eval_context::Value, syntax::Zero, types::Type};

    #[test]
    fn eval_fix() {
        let result = Fix {
            term: Box::new(EvalContext::Value(Value::Lambda {
                var: "x".to_owned(),
                annot: Type::Nat,
                body: Zero.into(),
            })),
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
