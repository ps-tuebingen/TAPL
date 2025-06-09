use super::types::Type;
use derivation::latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{App, False, Lambda, Num, True, TyApp, TyLambda, Unit, Variable},
    TypeVar, Var,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Num(Num<Term>),
    True(True<Term>),
    False(False<Term>),
    Unit(Unit<Term>),
    Lambda(Lambda<Term, Type>),
    TyLambda(TyLambda<Term>),
    App(App<Term>),
    TyApp(TyApp<Term, Type>),
}

impl syntax::terms::Term for Term {}

impl SubstType<Type> for Term {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty),
            Term::Num(num) => num.subst_type(v, ty),
            Term::True(tru) => tru.subst_type(v, ty),
            Term::False(fls) => fls.subst_type(v, ty),
            Term::Lambda(lam) => lam.subst_type(v, ty),
            Term::App(app) => app.subst_type(v, ty),
            Term::Unit(u) => u.subst_type(v, ty),
            Term::TyLambda(tylam) => tylam.subst_type(v, ty),
            Term::TyApp(tyapp) => tyapp.subst_type(v, ty),
        }
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::TyLambda(tylam) => tylam.subst(v, t),
            Term::TyApp(tyapp) => tyapp.subst(v, t),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::TyLambda(tylam) => tylam.fmt(f),
            Term::TyApp(tyapp) => tyapp.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::TyLambda(tylam) => tylam.to_latex(conf),
            Term::TyApp(tyapp) => tyapp.to_latex(conf),
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
    fn from(u: Unit<Term>) -> Term {
        Term::Unit(u)
    }
}

impl From<TyLambda<Term>> for Term {
    fn from(tylam: TyLambda<Term>) -> Term {
        Term::TyLambda(tylam)
    }
}

impl From<TyApp<Term, Type>> for Term {
    fn from(tyapp: TyApp<Term, Type>) -> Term {
        Term::TyApp(tyapp)
    }
}
