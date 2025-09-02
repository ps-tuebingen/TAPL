use super::{Exceptions, types::Type};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, True, Try, TryWithVal,
        Unit, Variable,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Exceptions>),
    Num(Num<Exceptions>),
    True(True<Exceptions>),
    False(False<Exceptions>),
    Succ(Succ<Exceptions>),
    Pred(Pred<Exceptions>),
    IsZero(IsZero<Exceptions>),
    If(If<Exceptions>),
    Lambda(Lambda<Exceptions>),
    App(App<Exceptions>),
    Unit(Unit<Exceptions>),
    Exception(Exception<Exceptions>),
    Try(Try<Exceptions>),
    Raise(Raise<Exceptions>),
    TryWithVal(TryWithVal<Exceptions>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Exceptions>::rule(),
            Num::<Exceptions>::rule(),
            True::<Exceptions>::rule(),
            False::<Exceptions>::rule(),
            Succ::<Exceptions>::rule(),
            Pred::<Exceptions>::rule(),
            IsZero::<Exceptions>::rule(),
            If::<Exceptions>::rule(),
            Lambda::<Exceptions>::rule(),
            App::<Exceptions>::rule(),
            Unit::<Exceptions>::rule(),
            Exception::<Exceptions>::rule(),
            Try::<Exceptions>::rule(),
            Raise::<Exceptions>::rule(),
            TryWithVal::<Exceptions>::rule(),
        ])
    }
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

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(v) => v.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::Exception(exc) => exc.to_latex(conf),
            Term::Try(t) => t.to_latex(conf),
            Term::Raise(r) => r.to_latex(conf),
            Term::TryWithVal(t) => t.to_latex(conf),
        }
    }
}

impl SubstTerm for Term {
    type Lang = Exceptions;
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::Exception(exc) => exc.subst(v, t).into(),
            Term::Try(tryt) => tryt.subst(v, t).into(),
            Term::Raise(raise) => raise.subst(v, t).into(),
            Term::TryWithVal(tryval) => tryval.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = Exceptions;
    type Target = Term;
    fn subst_type(self, v: &TypeVar, t: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, t).into(),
            Term::Num(num) => num.subst_type(v, t).into(),
            Term::True(tru) => tru.subst_type(v, t).into(),
            Term::False(fls) => fls.subst_type(v, t).into(),
            Term::Succ(succ) => succ.subst_type(v, t).into(),
            Term::Pred(pred) => pred.subst_type(v, t).into(),
            Term::IsZero(isz) => isz.subst_type(v, t).into(),
            Term::If(ift) => ift.subst_type(v, t).into(),
            Term::Lambda(lam) => lam.subst_type(v, t).into(),
            Term::App(app) => app.subst_type(v, t).into(),
            Term::Unit(u) => u.subst_type(v, t).into(),
            Term::Exception(exc) => exc.subst_type(v, t).into(),
            Term::Try(tryt) => tryt.subst_type(v, t).into(),
            Term::Raise(raise) => raise.subst_type(v, t).into(),
            Term::TryWithVal(tryval) => tryval.subst_type(v, t).into(),
        }
    }
}

impl From<Variable<Exceptions>> for Term {
    fn from(var: Variable<Exceptions>) -> Term {
        Term::Var(var)
    }
}

impl From<Num<Exceptions>> for Term {
    fn from(num: Num<Exceptions>) -> Term {
        Term::Num(num)
    }
}

impl From<True<Exceptions>> for Term {
    fn from(tru: True<Exceptions>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Exceptions>> for Term {
    fn from(fls: False<Exceptions>) -> Term {
        Term::False(fls)
    }
}

impl From<Succ<Exceptions>> for Term {
    fn from(succ: Succ<Exceptions>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<Exceptions>> for Term {
    fn from(pred: Pred<Exceptions>) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero<Exceptions>> for Term {
    fn from(isz: IsZero<Exceptions>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<If<Exceptions>> for Term {
    fn from(ift: If<Exceptions>) -> Term {
        Term::If(ift)
    }
}

impl From<Lambda<Exceptions>> for Term {
    fn from(lam: Lambda<Exceptions>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Exceptions>> for Term {
    fn from(app: App<Exceptions>) -> Term {
        Term::App(app)
    }
}

impl From<Unit<Exceptions>> for Term {
    fn from(unit: Unit<Exceptions>) -> Term {
        Term::Unit(unit)
    }
}

impl From<Exception<Exceptions>> for Term {
    fn from(exc: Exception<Exceptions>) -> Term {
        Term::Exception(exc)
    }
}

impl From<Try<Exceptions>> for Term {
    fn from(tryt: Try<Exceptions>) -> Term {
        Term::Try(tryt)
    }
}

impl From<Raise<Exceptions>> for Term {
    fn from(raise: Raise<Exceptions>) -> Term {
        Term::Raise(raise)
    }
}

impl From<TryWithVal<Exceptions>> for Term {
    fn from(tryval: TryWithVal<Exceptions>) -> Term {
        Term::TryWithVal(tryval)
    }
}

#[cfg(test)]
pub mod term_tests {
    use super::{App, Exceptions, Lambda, Raise, Term, Try, TryWithVal, Unit, Variable};
    use syntax::types::Unit as UnitTy;

    pub fn example_term1() -> Term {
        Try::<Exceptions>::new(
            App::<Exceptions>::new(
                Lambda::<Exceptions>::new("x", UnitTy::new(), Variable::<Exceptions>::new("x")),
                Unit::<Exceptions>::new(),
            ),
            Unit::<Exceptions>::new(),
        )
        .into()
    }

    pub fn example_term2() -> Term {
        TryWithVal::<Exceptions>::new(
            Raise::<Exceptions>::new(Unit::<Exceptions>::new(), UnitTy::new(), UnitTy::new()),
            Lambda::<Exceptions>::new("x", UnitTy::new(), Unit::new()),
        )
        .into()
    }
}
