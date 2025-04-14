use crate::types::Type;
use common::terms::{
    App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, Term as TermTrait, True,
    Try, TryWithVal, Unit, Variable,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Num(Num<Term>),
    True(True<Term>),
    False(False<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    IsZero(IsZero<Term>),
    If(If<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    Unit(Unit<Term>),
    Exception(Exception<Term, Type>),
    Try(Try<Term>),
    Raise(Raise<Term, Type>),
    TryWithVal(TryWithVal<Term>),
}

impl TermTrait for Term {}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::Exception(exc) => exc.fmt(f),
            Term::Try(t) => t.fmt(f),
            Term::Raise(r) => r.fmt(f),
            Term::TryWithVal(t) => t.fmt(f),
        }
    }
}

#[cfg(test)]
pub mod term_tests {
    use super::{App, Lambda, Raise, Term, Try, TryWithVal, Unit};
    use crate::types::Type;

    pub fn example_term1() -> Term {
        Try::new(
            App::new(
                Lambda::new("x", Type::Unit, "x".into()).into(),
                Unit::new().into(),
            )
            .into(),
            Unit::new().into(),
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
