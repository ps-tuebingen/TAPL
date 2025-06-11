use super::types::Type;
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{App, Lambda, TyApp, TyLambda, Variable},
    TypeVar, Var,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    TyLambda(TyLambda<Term>),
    TyApp(TyApp<Term, Type>),
}

impl syntax::terms::Term for Term {}

impl SubstType<Type> for Term {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty),
            Term::Lambda(lam) => lam.subst_type(v, ty),
            Term::App(app) => app.subst_type(v, ty),
            Term::TyLambda(lam) => lam.subst_type(v, ty),
            Term::TyApp(app) => app.subst_type(v, ty),
        }
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::TyLambda(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::TyLambda(lam) => lam.fmt(f),
            Term::TyApp(app) => app.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::TyLambda(lam) => lam.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
        }
    }
}

impl From<Variable<Term>> for Term {
    fn from(var: Variable<Term>) -> Term {
        Term::Var(var)
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
