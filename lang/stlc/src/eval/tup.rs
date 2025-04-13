use super::{to_eval_err, Value};
use crate::syntax::{Proj, Tup};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for Tup {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let mut vals = vec![];
        for term in self.terms.into_iter() {
            let val = term.eval(env)?;
            vals.push(val);
        }
        Ok(Value::Tup(vals))
    }
}

impl Eval<'_> for Proj {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let tup_val = self.tup.clone().eval(env)?;
        if let Value::Tup(vals) = tup_val {
            vals.get(self.ind)
                .cloned()
                .ok_or(to_eval_err(ErrorKind::TermMismatch {
                    found: self.to_string(),
                    expected: format!("Tuple projection less than {}", vals.len()),
                }))
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: tup_val.to_string(),
                expected: "Tuple".to_owned(),
            }))
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
