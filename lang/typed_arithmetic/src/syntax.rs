use crate::errors::Error;
use std::fmt;

#[derive(Debug)]
pub enum Term {
    True,
    False,
    If {
        ifc: Box<Term>,
        thent: Box<Term>,
        elset: Box<Term>,
    },
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Nat,
    Bool,
}

impl Type {
    pub fn check_equal(self, other: Type) -> Result<(), Error> {
        if self == other {
            Ok(())
        } else {
            Err(Error::TypeMismatch {
                found: self,
                expected: other,
            })
        }
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Term {
        if i > 0 {
            Term::Succ(Box::new((i - 1).into()))
        } else {
            Term::Zero
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::True => f.write_str("true"),
            Term::False => f.write_str("false"),
            Term::Zero => f.write_str("zero"),
            Term::Succ(t) => write!(f, "succ({t})"),
            Term::Pred(t) => write!(f, "pred({t})"),
            Term::IsZero(t) => write!(f, "iszero({t})"),
            Term::If { ifc, thent, elset } => {
                write!(f, "if ({ifc}) {{ {thent} }} else {{ {elset} }}")
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Nat => f.write_str("Nat"),
            Type::Bool => f.write_str("Bool"),
        }
    }
}
