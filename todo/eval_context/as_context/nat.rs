use super::{AsContext, EvalContext};
use crate::{
    eval::to_eval_err,
    eval_context::{
        computation::{IsZeroNum, PredSucc, SuccPred},
        congruence, Value,
    },
    syntax::{IsZero, Pred, Succ, Zero},
};
use common::errors::{Error, ErrorKind};

impl AsContext for Zero {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::Zero))
    }
}

impl AsContext for Succ {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(Value::Succ(val)) => Ok(EvalContext::Value(Value::Succ(Box::new(Value::Succ(val))))),
            Ok(Value::Pred(val)) => Ok(SuccPred { val: *val }.into()),
            Ok(Value::Zero) => Ok(EvalContext::Value(Value::Succ(Box::new(Value::Zero)))),
            Ok(val) => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "Number".to_owned(),
            })),
            Err(_) => {
                let inner = self.term.to_context()?;
                Ok(congruence::Succ {
                    term: Box::new(inner),
                }
                .into())
            }
        }
    }
}

impl AsContext for Pred {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(Value::Succ(val)) => Ok(PredSucc { val: *val }.into()),
            Ok(Value::Pred(val)) => Ok(EvalContext::Value(Value::Pred(Box::new(Value::Pred(val))))),
            Ok(Value::Zero) => Ok(EvalContext::Value(Value::Pred(Box::new(Value::Zero)))),
            Ok(val) => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "Number".to_owned(),
            })),
            Err(_) => {
                let inner = self.term.to_context()?;
                Ok(congruence::Pred {
                    term: Box::new(inner),
                }
                .into())
            }
        }
    }
}

impl AsContext for IsZero {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(val) => Ok(IsZeroNum { num: val }.into()),
            Err(_) => {
                let inner = self.term.to_context()?;
                Ok(congruence::IsZero {
                    term: Box::new(inner),
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{AsContext, EvalContext, IsZero, Pred, Succ, Zero};
    use crate::{
        eval_context::{
            computation::{HeadList, IsZeroNum, PredSucc, SuccPred},
            congruence, Value,
        },
        syntax::{Cons, Head, Nil},
        types::Type,
    };

    #[test]
    fn ctx_zero() {
        let result = Zero.to_context().unwrap();
        let expected = EvalContext::Value(Value::Zero);
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_succ_succ() {
        let result = Succ {
            term: Box::new(
                Succ {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected =
            EvalContext::Value(Value::Succ(Box::new(Value::Succ(Box::new(Value::Zero)))));
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_succ_pred() {
        let result = Succ {
            term: Box::new(
                Pred {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = SuccPred { val: Value::Zero }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_succ_cong() {
        let result = Succ {
            term: Box::new(
                Head {
                    inner_type: Type::Nat,
                    list: Box::new(
                        Cons {
                            fst: Box::new(Zero.into()),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Nat,
                                }
                                .into(),
                            ),
                            inner_type: Type::Nat,
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Succ {
            term: Box::new(
                HeadList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Nat,
                        }),
                        inner_type: Type::Nat,
                    },
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_pred_pred() {
        let result = Pred {
            term: Box::new(
                Pred {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected =
            EvalContext::Value(Value::Pred(Box::new(Value::Pred(Box::new(Value::Zero)))));
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_pred_succ() {
        let result = Pred {
            term: Box::new(
                Succ {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = PredSucc { val: Value::Zero }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_pred_cong() {
        let result = Pred {
            term: Box::new(
                Head {
                    inner_type: Type::Nat,
                    list: Box::new(
                        Cons {
                            fst: Box::new(Zero.into()),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Nat,
                                }
                                .into(),
                            ),
                            inner_type: Type::Nat,
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Pred {
            term: Box::new(
                HeadList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Nat,
                        }),
                        inner_type: Type::Nat,
                    },
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_iszero_val() {
        let result = IsZero {
            term: Box::new(Zero.into()),
        }
        .to_context()
        .unwrap();
        let expected = IsZeroNum { num: Value::Zero }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_iszero_cong() {
        let result = IsZero {
            term: Box::new(
                Head {
                    inner_type: Type::Nat,
                    list: Box::new(
                        Cons {
                            fst: Box::new(Zero.into()),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Nat,
                                }
                                .into(),
                            ),
                            inner_type: Type::Nat,
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::IsZero {
            term: Box::new(
                HeadList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Nat,
                        }),
                        inner_type: Type::Nat,
                    },
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
