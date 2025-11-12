use super::{SystemF, types::Type};
use latex::{LatexConfig, LatexFmt};
use macros::{Eval, GrammarDescribe, TermDisplay, Typecheck};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{App, Lambda, TyApp, TyLambda, Variable},
};

#[derive(TermDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(SystemF)]
pub enum Term {
    Variable(Variable<SystemF>),
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
            Term::Variable(var) => var.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::TyLambda(lam) => lam.subst_type(v, ty).into(),
            Term::TyApp(app) => app.subst_type(v, ty).into(),
        }
    }
}

impl SubstTerm for Term {
    type Lang = SystemF;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::TyLambda(lam) => lam.subst(v, t).into(),
            Term::TyApp(app) => app.subst(v, t).into(),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Variable(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::TyLambda(lam) => lam.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
        }
    }
}

impl From<Variable<SystemF>> for Term {
    fn from(var: Variable<SystemF>) -> Term {
        Term::Variable(var)
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
