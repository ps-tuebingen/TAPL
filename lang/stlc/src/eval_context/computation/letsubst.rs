use super::{ComputationRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term, traits::subst::Subst, Var};

#[derive(Debug, PartialEq, Eq)]
pub struct LetSubst {
    pub var: Var,
    pub bound_val: Value,
    pub in_term: Term,
}

impl Eval for LetSubst {
    fn eval(self) -> Result<Value, Error> {
        let t = self.in_term.subst(&self.var, self.bound_val.into());
        let ctx: EvalContext = t.to_context()?;
        println!("ctx after substitution {ctx:?}");
        ctx.eval()
    }
}

impl From<LetSubst> for ComputationRule {
    fn from(lt: LetSubst) -> ComputationRule {
        ComputationRule::LetSubst(lt)
    }
}

impl From<LetSubst> for EvalContext {
    fn from(lt: LetSubst) -> EvalContext {
        EvalContext::Computation(lt.into())
    }
}

#[cfg(test)]
mod letsubst_tests {
    use super::{Eval, LetSubst};
    use crate::{eval_context::Value, syntax::IsZero};

    #[test]
    fn eval_letsubst() {
        let result = LetSubst {
            var: "x".to_owned(),
            bound_val: Value::Zero,
            in_term: IsZero {
                term: Box::new("x".to_owned().into()),
            }
            .into(),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
