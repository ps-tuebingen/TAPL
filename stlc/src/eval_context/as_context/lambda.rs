use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{
        computation::AppAbs,
        congruence::{App1, App2},
        Value,
    },
    syntax::{App, Lambda},
};

impl AsContext for Lambda {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        }))
    }
}
impl AsContext for App {
    fn to_context(self) -> Result<EvalContext, Error> {
        match ((&*self.fun).try_into(), (&*self.arg).try_into()) {
            (Ok(fun_val), Err(_)) => {
                let arg_ctx = (*self.arg).to_context()?;
                Ok(App2 {
                    fun: fun_val,
                    arg: Box::new(arg_ctx),
                }
                .into())
            }
            (Err(_), _) => {
                let fun_ctx = (*self.fun).to_context()?;
                Ok(App1 {
                    fun: Box::new(fun_ctx),
                    arg: *self.arg,
                }
                .into())
            }
            (Ok(fun_val), Ok(arg_val)) => Ok(AppAbs {
                fun: fun_val,
                arg: arg_val,
            }
            .into()),
        }
    }
}

#[cfg(test)]
mod lam_tests {
    use super::{App, AsContext, EvalContext, Lambda, Value};
    use crate::{
        eval_context::{
            computation::{AppAbs, SuccPred},
            congruence::{App1, App2},
        },
        syntax::{Ascribe, Pred, Succ, Zero},
        types::Type,
    };

    #[test]
    fn ctx_lam() {
        let result = Lambda {
            var: "x".to_owned(),
            annot: Type::Nat,
            body: Box::new("x".to_owned().into()),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Lambda {
            var: "x".to_owned(),
            annot: Type::Nat,
            body: "x".to_owned().into(),
        });
        assert_eq!(result, expected)
    }
    #[test]
    fn ctx_app1() {
        let result = App {
            fun: Box::new(
                Ascribe {
                    term: Box::new(
                        Lambda {
                            var: "x".to_owned(),
                            annot: Type::Nat,
                            body: Box::new("x".to_owned().into()),
                        }
                        .into(),
                    ),
                    ty: Type::Fun(Box::new(Type::Nat), Box::new(Type::Nat)),
                }
                .into(),
            ),
            arg: Box::new(Zero.into()),
        }
        .to_context()
        .unwrap();
        let expected = App1 {
            fun: Box::new(
                EvalContext::Value(Value::Lambda {
                    var: "x".to_owned(),
                    annot: Type::Nat,
                    body: "x".to_owned().into(),
                })
                .into(),
            ),
            arg: Zero.into(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_app2() {
        let result = App {
            fun: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    annot: Type::Nat,
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            arg: Box::new(
                Succ {
                    term: Box::new(
                        Pred {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = App2 {
            fun: Value::Lambda {
                var: "x".to_owned(),
                annot: Type::Nat,
                body: "x".to_owned().into(),
            },
            arg: Box::new(SuccPred { val: Value::Zero }.into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_appabs() {
        let result = App {
            fun: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    annot: Type::Nat,
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            arg: Box::new(Zero.into()),
        }
        .to_context()
        .unwrap();
        let expected = AppAbs {
            fun: Value::Lambda {
                var: "x".to_owned(),
                annot: Type::Nat,
                body: "x".to_owned().into(),
            },
            arg: Value::Zero,
        }
        .into();
        assert_eq!(result, expected)
    }
}
