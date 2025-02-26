use super::{errors::Error, Eval, Value};
use crate::syntax::{IsZero, Pred, Succ, Zero};

impl Eval for Zero {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Zero)
    }
}
impl Eval for Succ {
    fn eval(self) -> Result<Value, Error> {
        let inner_res = self.term.eval()?;
        match inner_res {
            Value::Zero => Ok(Value::Succ(Box::new(Value::Zero))),
            Value::Succ(nv) => Ok(Value::Succ(Box::new(Value::Succ(nv)))),
            Value::Pred(nv) => Ok(*nv),
            _ => Err(Error::BadValue { val: inner_res }),
        }
    }
}

impl Eval for Pred {
    fn eval(self) -> Result<Value, Error> {
        let inner_res = self.term.eval()?;
        match inner_res {
            Value::Zero => Ok(Value::Pred(Box::new(Value::Zero))),
            Value::Succ(nv) => Ok(*nv),
            Value::Pred(nv) => Ok(Value::Pred(Box::new(Value::Pred(nv)))),
            _ => Err(Error::BadValue { val: inner_res }),
        }
    }
}

impl Eval for IsZero {
    fn eval(self) -> Result<Value, Error> {
        let inner_res = self.term.eval()?;
        match inner_res {
            Value::Zero => Ok(Value::True),
            Value::Succ(_) => Ok(Value::False),
            Value::Pred(_) => Ok(Value::False),
            _ => Err(Error::BadValue { val: inner_res }),
        }
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{Eval, IsZero, Pred, Succ, Value, Zero};

    #[test]
    fn eval_zero() {
        let result = Zero.eval().unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_succ_succ() {
        let result = Succ {
            term: Box::new(
                Succ {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Succ(Box::new(Value::Succ(Box::new(Value::Zero))));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_succ_pred() {
        let result = Succ {
            term: Box::new(
                Pred {
                    term: Box::new(
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
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_pred_succ() {
        let result = Pred {
            term: Box::new(
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
        .eval()
        .unwrap();
        let expected = Value::Pred(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_pred_pred() {
        let result = Pred {
            term: Box::new(
                Pred {
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
        .eval()
        .unwrap();
        let expected = Value::Pred(Box::new(Value::Pred(Box::new(Value::Pred(Box::new(
            Value::Zero,
        ))))));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_iszero_zero() {
        let result = IsZero {
            term: Box::new(Zero.into()),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_iszero_succ() {
        let result = IsZero {
            term: Box::new(
                Succ {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_iszero_pred() {
        let result = IsZero {
            term: Box::new(
                Pred {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }
}
