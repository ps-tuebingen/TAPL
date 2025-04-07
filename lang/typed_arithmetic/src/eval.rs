use crate::{errors::Error, syntax::Term};
use std::fmt;

pub fn eval(t: Term) -> Result<Value, Error> {
    match t {
        Term::Zero => Ok(NumericalValue::Zero.into()),
        Term::True => Ok(Value::True),
        Term::False => Ok(Value::False),
        Term::Succ(t) => {
            let inner_val = eval(*t)?;
            let nv = inner_val.as_numerical()?;
            match nv {
                NumericalValue::Zero => {
                    Ok(NumericalValue::Succ(Box::new(NumericalValue::Zero)).into())
                }
                NumericalValue::Succ(v) => {
                    Ok(NumericalValue::Succ(Box::new(NumericalValue::Succ(v))).into())
                }
            }
        }
        Term::Pred(t) => {
            let inner_val = eval(*t)?;
            let nv = inner_val.as_numerical()?;
            match nv {
                NumericalValue::Zero => Ok(NumericalValue::Zero.into()),
                NumericalValue::Succ(v) => Ok((*v).into()),
            }
        }
        Term::IsZero(t) => {
            let inner_val = eval(*t)?;
            let nv = inner_val.as_numerical()?;
            match nv {
                NumericalValue::Zero => Ok(Value::True),
                NumericalValue::Succ(_) => Ok(Value::False),
            }
        }
        Term::If { ifc, thent, elset } => {
            let cond_val = eval(*ifc)?;
            match cond_val {
                Value::True => eval(*thent),
                Value::False => eval(*elset),
                Value::Numerical(_) => Err(Error::BadValue {
                    found: cond_val,
                    expected: "Boolean Value".to_owned(),
                }),
            }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    True,
    False,
    Numerical(NumericalValue),
}

impl Value {
    pub fn as_numerical(self) -> Result<NumericalValue, Error> {
        if let Value::Numerical(nv) = self {
            Ok(nv)
        } else {
            Err(Error::BadValue {
                found: self,
                expected: "Numerical Value".to_owned(),
            })
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::True => f.write_str("true"),
            Value::False => f.write_str("false"),
            Value::Numerical(nv) => nv.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum NumericalValue {
    Zero,
    Succ(Box<NumericalValue>),
}

impl From<NumericalValue> for Value {
    fn from(nv: NumericalValue) -> Value {
        Value::Numerical(nv)
    }
}

impl fmt::Display for NumericalValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumericalValue::Zero => f.write_str("zero"),
            NumericalValue::Succ(nv) => write!(f, "succ({nv})"),
        }
    }
}
