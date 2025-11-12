use super::LambdaOmega;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{App, False, Lambda, Num, True, TyApp, TyLambda, Unit, Variable};

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
#[Lang(LambdaOmega)]
pub enum Term {
    Variable(Variable<LambdaOmega>),
    Num(Num<LambdaOmega>),
    True(True<LambdaOmega>),
    False(False<LambdaOmega>),
    Unit(Unit<LambdaOmega>),
    Lambda(Lambda<LambdaOmega>),
    TyLambda(TyLambda<LambdaOmega>),
    App(App<LambdaOmega>),
    TyApp(TyApp<LambdaOmega>),
}

impl syntax::terms::Term for Term {}
