use super::{ComputationRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, traits::subst::Subst};

#[derive(Debug, PartialEq, Eq)]
pub struct AppAbs {
    pub fun: Value,
    pub arg: Value,
}

impl Eval for AppAbs {
    fn eval(self) -> Result<Value, Error> {
        if let Value::Lambda {
            var,
            annot: _,
            body,
        } = self.fun
        {
            let term = body.subst(&var, self.arg.into());
            let ctx: EvalContext = term.to_context()?;
            ctx.eval()
        } else {
            Err(Error::BadValue { val: self.fun })
        }
    }
}

impl From<AppAbs> for ComputationRule {
    fn from(app: AppAbs) -> ComputationRule {
        ComputationRule::AppAbs(app)
    }
}

impl From<AppAbs> for EvalContext {
    fn from(app: AppAbs) -> EvalContext {
        EvalContext::Computation(app.into())
    }
}

#[cfg(test)]
mod appabs_tests {
    use super::{AppAbs, Eval};
    use crate::{eval_context::Value, syntax::IsZero, types::Type};

    #[test]
    fn eval_appabs() {
        let result = AppAbs {
            fun: Value::Lambda {
                var: "x".to_owned(),
                annot: Type::Nat,
                body: IsZero {
                    term: Box::new("x".to_owned().into()),
                }
                .into(),
            },
            arg: Value::Zero,
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
