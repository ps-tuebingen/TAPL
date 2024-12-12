use super::{ComputationRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term, traits::subst::Subst, Var};

#[derive(Debug, PartialEq, Eq)]
pub struct SomeCaseRhs {
    pub bound_val: Value,
    pub none_term: Term,
    pub some_var: Var,
    pub some_term: Term,
}

impl Eval for SomeCaseRhs {
    fn eval(self) -> Result<Value, Error> {
        match self.bound_val {
            Value::Nothing { .. } => {
                let ctx: EvalContext = self.none_term.to_context()?;
                ctx.eval()
            }
            Value::Something(val) => {
                let t = self.some_term.subst(&self.some_var, (*val).into());
                let ctx = t.to_context()?;
                ctx.eval()
            }
            val => Err(Error::BadValue { val }),
        }
    }
}

impl From<SomeCaseRhs> for ComputationRule {
    fn from(case: SomeCaseRhs) -> ComputationRule {
        ComputationRule::SomeCaseRhs(case)
    }
}

impl From<SomeCaseRhs> for EvalContext {
    fn from(case: SomeCaseRhs) -> EvalContext {
        EvalContext::Computation(case.into())
    }
}

#[cfg(test)]
mod somecaserhs_tests {
    use super::{Eval, SomeCaseRhs};
    use crate::{eval_context::Value, syntax::False, types::Type};

    #[test]
    fn eval_somecaserhs_some() {
        let result = SomeCaseRhs {
            bound_val: Value::Something(Box::new(Value::True)),
            none_term: False.into(),
            some_var: "x".to_owned(),
            some_term: "x".to_owned().into(),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_somecaserhs_none() {
        let result = SomeCaseRhs {
            bound_val: Value::Nothing {
                inner_type: Type::Bool,
            },
            none_term: False.into(),
            some_var: "x".to_owned(),
            some_term: "x".to_owned().into(),
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
