use super::Term;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Zero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Succ {
    pub term: Box<Term>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pred {
    pub term: Box<Term>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IsZero {
    pub term: Box<Term>,
}

impl From<i64> for Term {
    fn from(i: i64) -> Term {
        if i == 0 {
            Zero.into()
        } else if i > 0 {
            Succ {
                term: Box::new((i - 1).into()),
            }
            .into()
        } else {
            Pred {
                term: Box::new((i + 1).into()),
            }
            .into()
        }
    }
}

impl From<Zero> for Term {
    fn from(z: Zero) -> Term {
        Term::Zero(z)
    }
}

impl From<Pred> for Term {
    fn from(p: Pred) -> Term {
        Term::Pred(p)
    }
}

impl From<Succ> for Term {
    fn from(s: Succ) -> Term {
        Term::Succ(s)
    }
}

impl From<IsZero> for Term {
    fn from(isz: IsZero) -> Term {
        Term::IsZero(isz)
    }
}

impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("0")
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}

impl fmt::Display for Succ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}

impl fmt::Display for IsZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{Pred, Succ, Term, Zero};
    #[test]
    fn from_three() {
        let result: Term = 3.into();
        let expected = Succ {
            term: Box::new(
                Succ {
                    term: Box::new(
                        Succ {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn from_neg() {
        let result: Term = (-3).into();
        let expected = Pred {
            term: Box::new(
                Pred {
                    term: Box::new(
                        Pred {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn from_zero() {
        let result: Term = 0.into();
        let expected = Zero.into();
        assert_eq!(result, expected)
    }
}
