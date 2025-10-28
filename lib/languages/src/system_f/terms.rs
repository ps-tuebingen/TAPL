use super::{SystemF, types::Type};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{App, Lambda, TyApp, TyLambda, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<SystemF>),
    Lambda(Lambda<SystemF>),
    App(App<SystemF>),
    TyLambda(TyLambda<SystemF>),
    TyApp(TyApp<SystemF>),
}

impl syntax::terms::Term for Term {}

impl SubstType for Term {
    type Lang = SystemF;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::TyLambda(lam) => lam.subst_type(v, ty).into(),
            Term::TyApp(app) => app.subst_type(v, ty).into(),
        }
    }
}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<SystemF>::rule(),
            Lambda::<SystemF>::rule(),
            App::<SystemF>::rule(),
            TyLambda::<SystemF>::rule(),
            TyApp::<SystemF>::rule(),
        ])
    }
}

impl SubstTerm for Term {
    type Lang = SystemF;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::TyLambda(lam) => lam.subst(v, t).into(),
            Term::TyApp(app) => app.subst(v, t).into(),
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

impl From<Variable<SystemF>> for Term {
    fn from(var: Variable<SystemF>) -> Term {
        Term::Var(var)
    }
}
impl From<Lambda<SystemF>> for Term {
    fn from(lam: Lambda<SystemF>) -> Term {
        Term::Lambda(lam)
    }
}
impl From<App<SystemF>> for Term {
    fn from(app: App<SystemF>) -> Term {
        Term::App(app)
    }
}
impl From<TyLambda<SystemF>> for Term {
    fn from(tylam: TyLambda<SystemF>) -> Term {
        Term::TyLambda(tylam)
    }
}
impl From<TyApp<SystemF>> for Term {
    fn from(tyapp: TyApp<SystemF>) -> Term {
        Term::TyApp(tyapp)
    }
}
