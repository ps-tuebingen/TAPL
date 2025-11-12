use super::SystemF;
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck};
use syntax::terms::{App, Lambda, TyApp, TyLambda, Variable};

#[derive(
    SubstType,
    SubstTerm,
    LatexFmt,
    LangDisplay,
    GrammarDescribe,
    Eval,
    Typecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
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
