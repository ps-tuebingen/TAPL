use crate::{syntax::Term, to_err};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Eval,
};
use std::fmt;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

#[derive(Debug)]
pub enum Value {
    True,
    False,
    Numerical(i64),
}

impl Value {
    pub fn as_numerical(self) -> Result<i64, ErrorKind> {
        if let Value::Numerical(i) = self {
            Ok(i)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Number".to_owned(),
            })
        }
    }
}

impl Eval<'_> for Term {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        match self {
            Term::Zero => Ok(Value::Numerical(0)),
            Term::True => Ok(Value::True),
            Term::False => Ok(Value::False),
            Term::Succ(t) => {
                let inner_val = t.eval(_env)?;
                let nv = inner_val.as_numerical().map_err(to_eval_err)?;
                Ok(Value::Numerical(nv + 1))
            }
            Term::Pred(t) => {
                let inner_val = t.eval(_env)?;
                let nv = inner_val.as_numerical().map_err(to_eval_err)?;
                Ok(Value::Numerical(nv - 1))
            }
            Term::IsZero(t) => {
                let inner_val = t.eval(_env)?;
                let nv = inner_val.as_numerical().map_err(to_eval_err)?;
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
                    Value::Numerical(_) => Err(to_eval_err(ErrorKind::ValueMismatch {
                        found: cond_val.to_string(),
                        expected: "Boolean Value".to_owned(),
                    })),
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
