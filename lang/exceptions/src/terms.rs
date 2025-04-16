use crate::{types::Type, values::Value};
use common::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, True, Try, TryWithVal,
        Unit, Variable,
    },
    TypeVar, Var,
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
    Lambda(Lambda<Term>),
    App(App<Term>),
    Unit(Unit<Term>),
    Exception(Exception<Term>),
    Try(Try<Term>),
    Raise(Raise<Term>),
    TryWithVal(TryWithVal<Term>),
}

impl common::terms::Term for Term {}

impl LanguageTerm for Term {
    type Type = Type;
    type Value = Value;
}

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

impl SubstTerm<Term> for Term {
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::Exception(exc) => exc.subst(v, t),
            Term::Try(tryt) => tryt.subst(v, t),
            Term::Raise(raise) => raise.subst(v, t),
            Term::TryWithVal(tryval) => tryval.subst(v, t),
        }
    }
}

impl SubstType<Type> for Term {
    type Target = Term;
    fn subst_type(self, v: &TypeVar, t: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, t),
            Term::Num(num) => num.subst_type(v, t),
            Term::True(tru) => tru.subst_type(v, t),
            Term::False(fls) => fls.subst_type(v, t),
            Term::Succ(succ) => succ.subst_type(v, t),
            Term::Pred(pred) => pred.subst_type(v, t),
            Term::IsZero(isz) => isz.subst_type(v, t),
            Term::If(ift) => ift.subst_type(v, t),
            Term::Lambda(lam) => lam.subst_type(v, t),
            Term::App(app) => app.subst_type(v, t),
            Term::Unit(u) => u.subst_type(v, t),
            Term::Exception(exc) => exc.subst_type(v, t),
            Term::Try(tryt) => tryt.subst_type(v, t),
            Term::Raise(raise) => raise.subst_type(v, t),
            Term::TryWithVal(tryval) => tryval.subst_type(v, t),
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

impl From<Lambda<Term>> for Term {
    fn from(lam: Lambda<Term>) -> Term {
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

impl From<Exception<Term>> for Term {
    fn from(exc: Exception<Term>) -> Term {
        Term::Exception(exc)
    }
}

impl From<Try<Term>> for Term {
    fn from(tryt: Try<Term>) -> Term {
        Term::Try(tryt)
    }
}

impl From<Raise<Term>> for Term {
    fn from(raise: Raise<Term>) -> Term {
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
    use common::types::Unit as UnitTy;

    pub fn example_term1() -> Term {
        Try::<Term>::new(
            App::<Term>::new(
                Lambda::<Term>::new("x", UnitTy, Variable::<Term>::new("x")),
                Unit::<Term>::new(),
            ),
            Unit::<Term>::new(),
        )
        .into()
    }

    pub fn example_term2() -> Term {
        TryWithVal::<Term>::new(
            Raise::<Term>::new(Unit::<Term>::new(), UnitTy, UnitTy),
            Lambda::<Term>::new("x", UnitTy, Unit::new()),
        )
        .into()
    }
}
