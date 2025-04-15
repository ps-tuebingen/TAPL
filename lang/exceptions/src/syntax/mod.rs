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

impl From<Variable<Term>> for Term {
    fn from(var: Variable<Term>) -> Term {
        Term::Var(var)
    }
}

impl From<Num<Term>> for Term {
    fn from(num: Num<Term>) -> Term {
        Term::Num(num)
    }
}

impl From<True<Term>> for Term {
    fn from(tru: True<Term>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Term>> for Term {
    fn from(fls: False<Term>) -> Term {
        Term::False(fls)
    }
}

impl From<Succ<Term>> for Term {
    fn from(succ: Succ<Term>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<Term>> for Term {
    fn from(pred: Pred<Term>) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero<Term>> for Term {
    fn from(isz: IsZero<Term>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<If<Term>> for Term {
    fn from(ift: If<Term>) -> Term {
        Term::If(ift)
    }
}

impl From<Lambda<Term, Type>> for Term {
    fn from(lam: Lambda<Term, Type>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

impl From<Unit<Term>> for Term {
    fn from(unit: Unit<Term>) -> Term {
        Term::Unit(unit)
    }
}

impl From<Exception<Term, Type>> for Term {
    fn from(exc: Exception<Term, Type>) -> Term {
        Term::Exception(exc)
    }
}

impl From<Try<Term>> for Term {
    fn from(tryt: Try<Term>) -> Term {
        Term::Try(tryt)
    }
}

impl From<Raise<Term, Type>> for Term {
    fn from(raise: Raise<Term, Type>) -> Term {
        Term::Raise(raise)
    }
}

impl From<TryWithVal<Term>> for Term {
    fn from(tryval: TryWithVal<Term>) -> Term {
        Term::TryWithVal(tryval)
    }
}

#[cfg(test)]
pub mod term_tests {
    use super::{App, Lambda, Raise, Term, Try, TryWithVal, Unit, Variable};
    use crate::types::Type;
    use common::types::Unit as UnitTy;

    pub fn example_term1() -> Term {
        Try::<Term>::new(
            App::<Term>::new(
                Lambda::<Term, Type>::new("x", UnitTy.into(), Variable::<Term>::new("x").into())
                    .into(),
                Unit::<Term>::new().into(),
            )
            .into(),
            Unit::<Term>::new().into(),
        )
        .into()
    }

    pub fn example_term2() -> Term {
        TryWithVal::<Term>::new(
            Raise::<Term, Type>::new(Unit::<Term>::new().into(), UnitTy.into(), UnitTy.into())
                .into(),
            Lambda::<Term, Type>::new("x", UnitTy::new().into(), Unit::new().into()).into(),
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
