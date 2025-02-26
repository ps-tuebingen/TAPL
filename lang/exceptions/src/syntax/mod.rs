use std::fmt;

pub type Var = String;

pub mod exception;
pub mod exception_val;
pub mod lambda;
pub mod unit;

pub use exception::{Error, Try};
pub use exception_val::{Raise, TryWithVal};
pub use lambda::{App, Lambda};
pub use unit::Unit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    Unit(Unit),
    Error(Error),
    Try(Try),
    Raise(Raise),
    TryWithVal(TryWithVal),
}

impl Term {
    pub fn is_value(&self) -> bool {
        matches!(self, Term::Lambda(_) | Term::Unit(_))
    }
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{v}"),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::Error(err) => err.fmt(f),
            Term::Try(t) => t.fmt(f),
            Term::Raise(r) => r.fmt(f),
            Term::TryWithVal(t) => t.fmt(f),
        }
    }
}

impl From<Var> for Term {
    fn from(v: Var) -> Term {
        Term::Var(v)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}

#[cfg(test)]
pub mod term_tests {
    use super::{App, Lambda, Raise, Term, Try, TryWithVal, Unit};
    use crate::types::Type;

    pub fn example_term1() -> Term {
        Try::new(
            App::new(Lambda::new("x", Type::Unit, "x".into()).into(), Unit.into()).into(),
            Unit.into(),
        )
        .into()
    }

    pub fn example_term2() -> Term {
        TryWithVal::new(
            Raise::new(Unit.into(), Type::Unit, Type::Unit).into(),
            Lambda::new("x", Type::Unit, Unit.into()).into(),
        )
        .into()
    }

    #[test]
    fn is_val1() {
        let result = example_term1().is_value();
        assert!(!result)
    }

    #[test]
    fn is_val2() {
        let result = example_term2().is_value();
        assert!(!result)
    }
}
