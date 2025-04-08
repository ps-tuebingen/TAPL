use super::{errors::Error, Value};
use crate::{
    syntax::{Fix, Lambda},
    traits::subst::Subst,
};
use common::Eval;

impl<'a> Eval<'a> for Fix {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let fix_val = self.term.eval(env)?;
        if let Value::Lambda { var, annot, body } = fix_val {
            body.clone()
                .subst(
                    &var,
                    Fix {
                        term: Box::new(
                            Lambda {
                                var: var.clone(),
                                annot,
                                body: Box::new(body),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                )
                .eval(env)
        } else {
            Err(Error::BadValue { val: fix_val })
        }
    }
}

#[cfg(test)]
mod fix_tests {
    use super::{Eval, Fix, Value};
    use crate::{
        syntax::{App, False, If, IsZero, Lambda, Pred, True, Zero},
        types::Type,
    };

    #[test]
    fn eval_iseven() {
        let result = App {
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
                                                            fun: Box::new("ie".to_owned().into()),
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
        .eval(Default::default())
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
