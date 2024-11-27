use super::{errors::Error, Eval, Value};
use crate::syntax::{Cons, Head, IsNil, Nil, Tail};

impl Eval for Nil {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Nil {
            inner_type: self.inner_type,
        })
    }
}

impl Eval for Cons {
    fn eval(self) -> Result<Value, Error> {
        let fst_val = self.fst.eval()?;
        let rst_val = self.rst.eval()?;
        Ok(Value::Cons {
            inner_type: self.inner_type,
            fst: Box::new(fst_val),
            rst: Box::new(rst_val),
        })
    }
}

impl Eval for IsNil {
    fn eval(self) -> Result<Value, Error> {
        match self.list.eval()? {
            Value::Nil { .. } => Ok(Value::True),
            Value::Cons { .. } => Ok(Value::False),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl Eval for Head {
    fn eval(self) -> Result<Value, Error> {
        match self.list.eval()? {
            Value::Cons { fst, .. } => Ok(*fst),
            Value::Nil { .. } => Err(Error::HeadOfEmptyList),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl Eval for Tail {
    fn eval(self) -> Result<Value, Error> {
        match self.list.eval()? {
            Value::Cons { rst, .. } => Ok(*rst),
            Value::Nil { .. } => Err(Error::TailOfEmptyList),
            val => Err(Error::BadValue { val }),
        }
    }
}

#[cfg(test)]
mod list_tests {
    use super::{Cons, Eval, Head, IsNil, Nil, Tail, Value};
    use crate::{syntax::True, types::Type};

    #[test]
    fn eval_nil() {
        let result = Nil {
            inner_type: Type::Bool,
        }
        .eval()
        .unwrap();
        let expected = Value::Nil {
            inner_type: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_cons() {
        let result = Cons {
            inner_type: Type::Bool,
            fst: Box::new(True.into()),
            rst: Box::new(
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Cons {
            inner_type: Type::Bool,
            fst: Box::new(Value::True),
            rst: Box::new(Value::Nil {
                inner_type: Type::Bool,
            }),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_isnil_nil() {
        let result = IsNil {
            inner_type: Type::Bool,
            list: Box::new(
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_isnil_cons() {
        let result = IsNil {
            inner_type: Type::Bool,
            list: Box::new(
                Cons {
                    inner_type: Type::Bool,
                    fst: Box::new(True.into()),
                    rst: Box::new(
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                    ),
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
    fn eval_head_nil() {
        let result = Head {
            inner_type: Type::Bool,
            list: Box::new(
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ),
        }
        .eval();
        assert!(result.is_err())
    }

    #[test]
    fn eval_head_cons() {
        let result = Head {
            inner_type: Type::Bool,
            list: Box::new(
                Cons {
                    inner_type: Type::Bool,
                    fst: Box::new(True.into()),
                    rst: Box::new(
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_tail_nil() {
        let result = Tail {
            inner_type: Type::Bool,
            list: Box::new(
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ),
        }
        .eval();
        assert!(result.is_err())
    }

    #[test]
    fn eval_tail_cons() {
        let result = Tail {
            inner_type: Type::Bool,
            list: Box::new(
                Cons {
                    inner_type: Type::Bool,
                    fst: Box::new(True.into()),
                    rst: Box::new(
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Nil {
            inner_type: Type::Bool,
        };
        assert_eq!(result, expected)
    }
}
