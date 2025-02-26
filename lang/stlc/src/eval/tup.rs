use super::{errors::Error, Eval, Value};
use crate::syntax::{Proj, Tup};

impl Eval for Tup {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = vec![];
        for term in self.terms.into_iter() {
            let val = term.eval()?;
            vals.push(val);
        }
        Ok(Value::Tup(vals))
    }
}

impl Eval for Proj {
    fn eval(self) -> Result<Value, Error> {
        let tup_val = self.tup.eval()?;
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
        .eval()
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
        .eval()
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
