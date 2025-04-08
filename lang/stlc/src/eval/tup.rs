use super::{errors::Error, Eval, Value};
use crate::syntax::{Proj, Tup};

impl<'a> Eval<'a> for Tup {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let mut vals = vec![];
        for term in self.terms.into_iter() {
            let val = term.eval(env)?;
            vals.push(val);
        }
        Ok(Value::Tup(vals))
    }
}

impl<'a> Eval<'a> for Proj {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let tup_val = self.tup.eval(env)?;
        if let Value::Tup(vals) = tup_val {
            vals.get(self.ind)
                .cloned()
                .ok_or(Error::ProjectionOutOfBounds {
                    found: vals.len(),
                    expected: self.ind,
                })
        } else {
            Err(Error::BadValue { val: tup_val })
        }
    }
}

#[cfg(test)]
mod tup_tests {
    use super::{Eval, Proj, Tup, Value};
    use crate::{
        syntax::{False, Nil, True, Zero},
        types::Type,
    };

    #[test]
    fn eval_tup() {
        let result = Tup {
            terms: vec![
                Zero.into(),
                True.into(),
                False.into(),
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ],
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Tup(vec![
            Value::Zero,
            Value::True,
            Value::False,
            Value::Nil {
                inner_type: Type::Bool,
            },
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_proj() {
        let result = Proj {
            tup: Box::new(
                Tup {
                    terms: vec![
                        Zero.into(),
                        True.into(),
                        False.into(),
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                    ],
                }
                .into(),
            ),
            ind: 0,
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
