use super::SystemF;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{App, Lambda, TyApp, TyLambda, Variable};

#[derive(
    FromVariants,
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
