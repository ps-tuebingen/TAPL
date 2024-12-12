use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{
        computation::{HeadList, IsNilList, TailList},
        congruence,
        congruence::{Cons1, Cons2},
        Value,
    },
    syntax::{Cons, Head, IsNil, Nil, Tail},
};

impl AsContext for Nil {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::Nil {
            inner_type: self.inner_type,
        }))
    }
}

impl AsContext for Cons {
    fn to_context(self) -> Result<EvalContext, Error> {
        match ((&*self.fst).try_into(), (&*self.rst).try_into()) {
            (Err(_), _) => {
                let ctx = (*self.fst).to_context()?;
                Ok(Cons1 {
                    inner_type: self.inner_type,
                    fst: Box::new(ctx),
                    rst: *self.rst,
                }
                .into())
            }
            (Ok(fst_val), Err(_)) => {
                let ctx = (*self.rst).to_context()?;
                Ok(Cons2 {
                    inner_type: self.inner_type,
                    fst: fst_val,
                    rst: Box::new(ctx),
                }
                .into())
            }
            (Ok(fst_val), Ok(rnd_val)) => Ok(EvalContext::Value(Value::Cons {
                fst: Box::new(fst_val),
                rst: Box::new(rnd_val),
                inner_type: self.inner_type,
            })),
        }
    }
}

impl AsContext for IsNil {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.list).try_into() {
            Ok(val) => Ok(IsNilList { list: val }.into()),
            Err(_) => {
                let ctx = (*self.list).to_context()?;
                Ok(congruence::IsNil {
                    inner_type: self.inner_type,
                    list: Box::new(ctx),
                }
                .into())
            }
        }
    }
}

impl AsContext for Head {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.list).try_into() {
            Ok(val) => Ok(HeadList { list: val }.into()),
            Err(_) => {
                let ctx = (*self.list).to_context()?;
                Ok(congruence::Head {
                    inner_type: self.inner_type,
                    list: Box::new(ctx),
                }
                .into())
            }
        }
    }
}

impl AsContext for Tail {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.list).try_into() {
            Ok(val) => Ok(TailList { list: val }.into()),
            Err(_) => {
                let ctx = (*self.list).to_context()?;
                Ok(congruence::Tail {
                    inner_type: self.inner_type,
                    list: Box::new(ctx),
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod list_tests {
    use super::{AsContext, Cons, EvalContext, Head, IsNil, Nil, Tail};
    use crate::{
        eval_context::{
            computation::{HeadList, IsNilList, SuccPred, TailList},
            congruence,
            congruence::{Cons1, Cons2},
            Value,
        },
        syntax::{Pred, Succ, Zero},
        types::Type,
    };

    #[test]
    fn ctx_nil() {
        let result = Nil {
            inner_type: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Nil {
            inner_type: Type::Nat,
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_cons1() {
        let result = Cons {
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
            rst: Box::new(
                Nil {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
            inner_type: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = Cons1 {
            fst: Box::new(SuccPred { val: Value::Zero }.into()),
            rst: Nil {
                inner_type: Type::Nat,
            }
            .into(),
            inner_type: Type::Nat,
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_cons2() {
        let result = Cons {
            fst: Box::new(Zero.into()),
            rst: Box::new(
                Tail {
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
                    inner_type: Type::Nat,
                }
                .into(),
            ),
            inner_type: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = Cons2 {
            fst: Value::Zero,
            inner_type: Type::Nat,
            rst: Box::new(
                TailList {
                    list: Value::Cons {
                        inner_type: Type::Nat,
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Nil {
                            inner_type: Type::Nat,
                        }),
                    },
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_consval() {
        let result = Cons {
            fst: Box::new(Zero.into()),
            inner_type: Type::Nat,
            rst: Box::new(
                Nil {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Cons {
            inner_type: Type::Nat,
            fst: Box::new(Value::Zero),
            rst: Box::new(Value::Nil {
                inner_type: Type::Nat,
            }),
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_isnilcong() {
        let result = IsNil {
            inner_type: Type::Nat,
            list: Box::new(
                Tail {
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
        let expected = congruence::IsNil {
            inner_type: Type::Nat,
            list: Box::new(
                TailList {
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
    fn ctx_isnilval() {
        let result = IsNil {
            inner_type: Type::Nat,
            list: Box::new(
                Nil {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = IsNilList {
            list: Value::Nil {
                inner_type: Type::Nat,
            },
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_headcong() {
        let result = Head {
            inner_type: Type::Nat,
            list: Box::new(
                Tail {
                    inner_type: Type::Nat,
                    list: Box::new(
                        Cons {
                            inner_type: Type::Nat,
                            fst: Box::new(Zero.into()),
                            rst: Box::new(
                                Nil {
                                    inner_type: Type::Nat,
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
        .to_context()
        .unwrap();
        let expected = congruence::Head {
            inner_type: Type::Nat,
            list: Box::new(
                TailList {
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
    fn ctx_headval() {
        let result = Head {
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
        .to_context()
        .unwrap();
        let expected = HeadList {
            list: Value::Cons {
                fst: Box::new(Value::Zero),
                rst: Box::new(Value::Nil {
                    inner_type: Type::Nat,
                }),
                inner_type: Type::Nat,
            },
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_tailcong() {
        let result = Tail {
            list: Box::new(
                Tail {
                    list: Box::new(
                        Cons {
                            fst: Box::new(Zero.into()),
                            rst: Box::new(
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
                            inner_type: Type::Nat,
                        }
                        .into(),
                    ),
                    inner_type: Type::Nat,
                }
                .into(),
            ),
            inner_type: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = congruence::Tail {
            list: Box::new(
                TailList {
                    list: Value::Cons {
                        fst: Box::new(Value::Zero),
                        rst: Box::new(Value::Cons {
                            fst: Box::new(Value::Zero),
                            rst: Box::new(Value::Nil {
                                inner_type: Type::Nat,
                            }),
                            inner_type: Type::Nat,
                        }),
                        inner_type: Type::Nat,
                    },
                }
                .into(),
            ),
            inner_type: Type::Nat,
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_tailval() {
        let result = Tail {
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
            inner_type: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = TailList {
            list: Value::Cons {
                fst: Box::new(Value::Zero),
                rst: Box::new(Value::Nil {
                    inner_type: Type::Nat,
                }),
                inner_type: Type::Nat,
            },
        }
        .into();
        assert_eq!(result, expected)
    }
}
