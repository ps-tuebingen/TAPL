use super::{app2::App2, CongruenceRule, Error, Eval, EvalContext, Value};
use crate::{eval_context::AsContext, syntax::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct App1 {
    pub fun: Box<EvalContext>,
    pub arg: Term,
}
impl Eval for App1 {
    fn eval(self) -> Result<Value, Error> {
        let val = self.fun.eval()?;
        let inner_ctx: EvalContext = self.arg.to_context()?;
        let ctx: EvalContext = App2 {
            fun: val,
            arg: Box::new(inner_ctx),
        }
        .into();
        ctx.eval()
    }
}

impl From<App1> for CongruenceRule {
    fn from(app1: App1) -> CongruenceRule {
        CongruenceRule::App1(app1)
    }
}

impl From<App1> for EvalContext {
    fn from(app1: App1) -> EvalContext {
        EvalContext::Congruence(app1.into())
    }
}

#[cfg(test)]
mod app1_tests {
    use super::{App1, Eval};
    use crate::{
        eval_context::{computation::IfBool, Value},
        syntax::{Lambda, Zero},
        types::Type,
    };

    #[test]
    fn eval_app1() {
        let result = App1 {
            fun: Box::new(
                IfBool {
                    ifc: Value::True,
                    thenc: Lambda {
                        var: "x".to_owned(),
                        annot: Type::Nat,
                        body: Box::new(Zero.into()),
                    }
                    .into(),

                    elsec: Lambda {
                        var: "y".to_owned(),
                        annot: Type::Nat,
                        body: Box::new("y".to_owned().into()),
                    }
                    .into(),
                }
                .into(),
            ),
            arg: Zero.into(),
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
