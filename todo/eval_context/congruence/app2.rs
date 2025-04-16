use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::AppAbs;

#[derive(Debug, PartialEq, Eq)]
pub struct App2 {
    pub fun: Value,
    pub arg: Box<EvalContext>,
}
impl Eval for App2 {
    fn eval(self) -> Result<Value, Error> {
        let val = self.arg.eval()?;
        let ctx: EvalContext = AppAbs {
            fun: self.fun,
            arg: val,
        }
        .into();
        ctx.eval()
    }
}

impl From<App2> for CongruenceRule {
    fn from(app2: App2) -> CongruenceRule {
        CongruenceRule::App2(app2)
    }
}

impl From<App2> for EvalContext {
    fn from(app2: App2) -> EvalContext {
        EvalContext::Congruence(app2.into())
    }
}

#[cfg(test)]
mod app2_tests {
    use super::{App2, Eval};
    use crate::{
        eval_context::{computation::IsZeroNum, Value},
        types::Type,
    };

    #[test]
    fn eval_app2() {
        let result = App2 {
            fun: Value::Lambda {
                var: "x".to_owned(),
                annot: Type::Bool,
                body: "x".to_owned().into(),
            }
            .into(),
            arg: Box::new(IsZeroNum { num: Value::Zero }.into()),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
