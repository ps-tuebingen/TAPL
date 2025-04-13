pub mod as_context;
pub mod computation;
pub mod congruence;
pub mod context;
pub mod value;

use common::errors::Error;

use computation::ComputationRule;
use congruence::CongruenceRule;
use context::EvalContext;
use value::Value;

pub use as_context::AsContext;

use crate::syntax::Term;

pub trait Eval {
    fn eval(self) -> Result<Value, Error>;
}

pub fn eval_with_context(t: Term) -> Result<Value, Error> {
    let ctx: EvalContext = t.to_context()?;
    ctx.eval()
}

#[cfg(test)]
mod eval_tests {
    use super::eval_with_context;
    use crate::{
        eval_context::Value,
        syntax::{App, False, Fix, If, IsZero, Lambda, Pred, True, Zero},
        types::Type,
    };
    #[test]
    fn eval_iseven() {
        let result = eval_with_context(
            App {
                fun: Box::new(
                    Fix {
                        term: Box::new(
                            Lambda {
                                var: "ie".to_owned(),
                                annot: Type::Fun(Box::new(Type::Nat), Box::new(Type::Bool)),
                                body: Box::new(
                                    Lambda {
                                        var: "x".to_owned(),
                                        annot: Type::Nat,
                                        body: Box::new(
                                            If {
                                                ifc: Box::new(
                                                    IsZero {
                                                        term: Box::new("x".to_owned().into()),
                                                    }
                                                    .into(),
                                                ),
                                                thenc: Box::new(True.into()),
                                                elsec: Box::new(
                                                    If {
                                                        ifc: Box::new(
                                                            IsZero {
                                                                term: Box::new(
                                                                    Pred {
                                                                        term: Box::new(
                                                                            "x".to_owned().into(),
                                                                        ),
                                                                    }
                                                                    .into(),
                                                                ),
                                                            }
                                                            .into(),
                                                        ),
                                                        thenc: Box::new(False.into()),
                                                        elsec: Box::new(
                                                            App {
                                                                fun: Box::new(
                                                                    "ie".to_owned().into(),
                                                                ),
                                                                arg: Box::new(
                                                                    Pred {
                                                                        term: Box::new(
                                                                            Pred {
                                                                                term: Box::new(
                                                                                    "x".to_owned()
                                                                                        .into(),
                                                                                ),
                                                                            }
                                                                            .into(),
                                                                        ),
                                                                    }
                                                                    .into(),
                                                                ),
                                                            }
                                                            .into(),
                                                        ),
                                                    }
                                                    .into(),
                                                ),
                                            }
                                            .into(),
                                        ),
                                    }
                                    .into(),
                                ),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
                arg: Box::new(Zero.into()),
            }
            .into(),
        )
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
