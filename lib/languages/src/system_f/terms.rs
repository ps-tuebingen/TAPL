use super::{SystemF, types::Type};
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::SubstType,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
};

#[derive(
    SubstTerm, LatexFmt, LangDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq,
)]
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
