use crate::{errors::Error, syntax::Term};
use common::Eval;
use std::fmt;

#[derive(Debug)]
pub enum Value {
    True,
    False,
    Numerical(i64),
}

impl Value {
    pub fn as_numerical(self) -> Result<i64, Error> {
        if let Value::Numerical(i) = self {
            Ok(i)
        } else {
            Err(Error::BadValue {
                found: self,
                expected: "Number".to_owned(),
            })
        }
    }
}

impl<'a> Eval<'a> for Term {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        match self {
            Term::Zero => Ok(Value::Numerical(0)),
            Term::True => Ok(Value::True),
            Term::False => Ok(Value::False),
            Term::Succ(t) => {
                let inner_val = t.eval(_env)?;
                let nv = inner_val.as_numerical()?;
                Ok(Value::Numerical(nv + 1))
            }
            Term::Pred(t) => {
                let inner_val = t.eval(_env)?;
                let nv = inner_val.as_numerical()?;
                Ok(Value::Numerical(nv - 1))
            }
            Term::IsZero(t) => {
                let inner_val = t.eval(_env)?;
                let nv = inner_val.as_numerical()?;
                if nv == 0 {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            Term::If { ifc, thent, elset } => {
                let cond_val = ifc.eval(_env)?;
                match cond_val {
                    Value::True => thent.eval(_env),
                    Value::False => elset.eval(_env),
                    Value::Numerical(_) => Err(Error::BadValue {
                        found: cond_val,
                        expected: "Boolean Value".to_owned(),
                    }),
                }
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::True => f.write_str("true"),
            Value::False => f.write_str("false"),
            Value::Numerical(i) => write!(f, "{i}"),
        }
    }
}
