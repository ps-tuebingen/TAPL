use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{
        computation::{Proj1Beta, Proj2Beta},
        congruence, Value,
    },
    syntax::{Pair, Proj1, Proj2},
};

impl AsContext for Pair {
    fn to_context(self) -> Result<EvalContext, Error> {
        match ((&*self.fst).try_into(), (&*self.snd).try_into()) {
            (Err(_), _) => {
                let fst_ctx = (*self.fst).to_context()?;
                Ok(congruence::Pair1 {
                    fst: Box::new(fst_ctx),
                    snd: *self.snd,
                }
                .into())
            }
            (Ok(fst_val), Err(_)) => {
                let snd_ctx = (*self.snd).to_context()?;
                Ok(congruence::Pair2 {
                    fst: fst_val,
                    snd: Box::new(snd_ctx),
                }
                .into())
            }
            (Ok(fst_val), Ok(snd_val)) => Ok(EvalContext::Value(Value::Pair {
                fst: Box::new(fst_val),
                snd: Box::new(snd_val),
            })),
        }
    }
}

impl AsContext for Proj1 {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.pair).try_into() {
            Ok(val) => Ok(Proj1Beta { pair: val }.into()),
            Err(_) => {
                let ctx = (*self.pair).to_context()?;
                Ok(congruence::Proj1 {
                    term: Box::new(ctx),
                }
                .into())
            }
        }
    }
}

impl AsContext for Proj2 {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.pair).try_into() {
            Ok(val) => Ok(Proj2Beta { pair: val }.into()),
            Err(_) => {
                let ctx = (*self.pair).to_context()?;
                Ok(congruence::Proj2 {
                    term: Box::new(ctx),
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod pair_tests {
    use super::{AsContext, EvalContext, Pair, Proj1, Proj2};
    use crate::{
        eval_context::{
            computation::{HeadList, IsZeroNum, Proj1Beta, Proj2Beta, SuccPred},
            congruence, Value,
        },
        syntax::{Cons, Head, IsZero, Nil, Pred, Succ, True, Zero},
        types::Type,
    };

    #[test]
    fn ctx_pair_val() {
        let result = Pair {
            fst: Box::new(Zero.into()),
            snd: Box::new(True.into()),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Pair {
            fst: Box::new(Value::Zero),
            snd: Box::new(Value::True),
        })
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_pair_fst() {
        let result = Pair {
            fst: Box::new(
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
            snd: Box::new(True.into()),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Pair1 {
            fst: Box::new(SuccPred { val: Value::Zero }.into()),
            snd: True.into(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_pair_snd() {
        let result = Pair {
            fst: Box::new(Zero.into()),
            snd: Box::new(
                IsZero {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Pair2 {
            fst: Value::Zero,
            snd: Box::new(IsZeroNum { num: Value::Zero }.into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj1_beta() {
        let result = Proj1 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = Proj1Beta {
            pair: Value::Pair {
                fst: Box::new(Value::Zero),
                snd: Box::new(Value::True),
            },
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj1_cong() {
        let result = Proj1 {
            pair: Box::new(
                Head {
                    inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                    list: Box::new(
                        Cons {
                            fst: Box::new(
                                Pair {
                                    fst: Box::new(Zero.into()),
                                    snd: Box::new(True.into()),
                                }
                                .into(),
                            ),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Prod(
                                        Box::new(Type::Nat),
                                        Box::new(Type::Bool),
                                    ),
                                }
                                .into(),
                            ),
                            inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Proj1 {
            term: Box::new(
                HeadList {
                    list: Value::Cons {
                        fst: Box::new(Value::Pair {
                            fst: Box::new(Value::Zero),
                            snd: Box::new(Value::True),
                        }),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                        }),
                        inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                    },
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj2_beta() {
        let result = Proj2 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(True.into()),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = Proj2Beta {
            pair: Value::Pair {
                fst: Box::new(Value::Zero),
                snd: Box::new(Value::True),
            },
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj2_cong() {
        let result = Proj2 {
            pair: Box::new(
                Head {
                    inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                    list: Box::new(
                        Cons {
                            fst: Box::new(
                                Pair {
                                    fst: Box::new(Zero.into()),
                                    snd: Box::new(True.into()),
                                }
                                .into(),
                            ),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Prod(
                                        Box::new(Type::Nat),
                                        Box::new(Type::Bool),
                                    ),
                                }
                                .into(),
                            ),
                            inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Proj2 {
            term: Box::new(
                HeadList {
                    list: Value::Cons {
                        fst: Box::new(Value::Pair {
                            fst: Box::new(Value::Zero),
                            snd: Box::new(Value::True),
                        }),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                        }),
                        inner_type: Type::Prod(Box::new(Type::Nat), Box::new(Type::Bool)),
                    },
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
