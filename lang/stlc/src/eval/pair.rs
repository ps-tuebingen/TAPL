use super::{errors::Error, Eval, Value};
use crate::syntax::{Pair, Proj1, Proj2};

impl Eval for Pair {
    fn eval(self) -> Result<Value, Error> {
        let val_1 = self.fst.eval()?;
        let val_2 = self.snd.eval()?;
        Ok(Value::Pair {
            fst: Box::new(val_1),
            snd: Box::new(val_2),
        })
    }
}

impl Eval for Proj1 {
    fn eval(self) -> Result<Value, Error> {
        match self.pair.eval()? {
            Value::Pair { fst: v1, .. } => Ok(*v1),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl Eval for Proj2 {
    fn eval(self) -> Result<Value, Error> {
        match self.pair.eval()? {
            Value::Pair { snd: v2, .. } => Ok(*v2),
            val => Err(Error::BadValue { val }),
        }
    }
}

#[cfg(test)]
mod pair_tests {
    use super::{Eval, Pair, Proj1, Proj2, Value};
    use crate::syntax::{Succ, Zero};

    #[test]
    fn eval_pair() {
        let result = Pair {
            fst: Box::new(Zero.into()),
            snd: Box::new(
                Succ {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }
        .eval()
        .unwrap();
        let expected = Value::Pair {
            fst: Box::new(Value::Zero.into()),
            snd: Box::new(Value::Succ(Box::new(Value::Zero))),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_proj1() {
        let result = Proj1 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(
                        Succ {
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
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_proj2() {
        let result = Proj2 {
            pair: Box::new(
                Pair {
                    fst: Box::new(Zero.into()),
                    snd: Box::new(
                        Succ {
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
        let expected = Value::Succ(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }
}
