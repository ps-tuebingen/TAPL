use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
    Eval,
};
use std::fmt;

pub mod bool;
pub mod inductive_definitions;
pub mod parse;

pub fn to_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Eval,
        lang: Lang::UntypedArithmetic,
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    True,
    False,
    Numerical(i64),
}

impl Value {
    fn into_numerical(self) -> Result<i64, ErrorKind> {
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::True => f.write_str("true"),
            Value::False => f.write_str("false"),
            Value::Numerical(i) => write!(f, "{i}"),
        }
    }
}

impl Eval<'_> for Term {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::True => Ok(Value::True),
            Term::False => Ok(Value::False),
            Term::Zero => Ok(Value::Numerical(0)),
            Term::IsZero(t) => {
                let val = t.eval(_env)?;
                let num = val.into_numerical().map_err(to_err)?;
                if num == 0 {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            Term::Succ(t) => {
                let val = t.eval(_env)?;
                let num = val.into_numerical().map_err(to_err)?;
                Ok(Value::Numerical(num + 1))
            }
            Term::Pred(t) => {
                let val = t.eval(_env)?;
                let num = val.into_numerical().map_err(to_err)?;
                Ok(Value::Numerical(num - 1))
            }
            Term::If(ifc, thent, elset) => {
                let val = ifc.eval(_env)?;
                match val {
                    Value::True => thent.eval(_env),
                    Value::False => elset.eval(_env),
                    _ => Err(to_err(ErrorKind::ValueMismatch {
                        found: val.to_string(),
                        expected: "Boolean Value".to_owned(),
                    })),
                }
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::True => f.write_str("true"),
            Term::False => f.write_str("false"),
            Term::If(ift, thent, elset) => write!(f, "if ({ift}) {{ {thent} }} else {{ {elset} }}"),
            Term::Zero => f.write_str("zero"),
            Term::Succ(t) => write!(f, "succ({t})"),
            Term::Pred(t) => write!(f, "pred({t})"),
            Term::IsZero(t) => write!(f, "iszero({t})"),
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::{Eval, Term, Value};

    #[test]
    fn eval_simple() {
        let result = Term::Succ(Box::new(Term::Succ(Box::new(Term::Pred(Box::new(
            Term::Zero,
        ))))))
        .eval(Default::default())
        .unwrap();
        let expected = Value::Numerical(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_complex() {
        let result = Term::If(
            Box::new(Term::IsZero(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Pred(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Succ(Box::new(Term::Pred(Box::new(Term::Zero))))),
        )
        .eval(Default::default())
        .unwrap();
        let expected = Value::Numerical(0);
        assert_eq!(result, expected)
    }
}
