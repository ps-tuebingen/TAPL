use common::Eval;
use std::fmt;

pub mod bool;
pub mod inductive_definitions;
pub mod parse;

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
    fn into_numerical(self) -> Result<i64, String> {
        if let Value::Numerical(i) = self {
            Ok(i)
        } else {
            Err(format!("Bad Value {self:?}"))
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

impl Eval for Term {
    type Value = Value;
    type Error = String;
    fn eval(self) -> Result<Self::Value, Self::Error> {
        match self {
            Term::True => Ok(Value::True),
            Term::False => Ok(Value::False),
            Term::Zero => Ok(Value::Numerical(0)),
            Term::IsZero(t) => {
                let val = t.eval()?;
                let num = val.into_numerical()?;
                if num == 0 {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            Term::Succ(t) => {
                let val = t.eval()?;
                let num = val.into_numerical()?;
                Ok(Value::Numerical(num + 1))
            }
            Term::Pred(t) => {
                let val = t.eval()?;
                let num = val.into_numerical()?;
                Ok(Value::Numerical(num - 1))
            }
            Term::If(ifc, thent, elset) => {
                let val = ifc.eval()?;
                match val {
                    Value::True => thent.eval(),
                    Value::False => elset.eval(),
                    _ => Err("If Condition needs to be boolean".to_owned()),
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
        .eval()
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
        .eval()
        .unwrap();
        let expected = Value::Numerical(0);
        assert_eq!(result, expected)
    }
}
