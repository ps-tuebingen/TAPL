use super::{SystemF, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{Lambda, TyLambda, Value as ValueTrait, ValueGroup};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    FromVariants,
    LatexFmt,
    LangDisplay,
    Debug,
    PartialEq,
    Eq,
    Clone,
)]
#[Lang(SystemF)]
pub enum Value {
    Lambda(Lambda<SystemF>),
    TyLambda(TyLambda<SystemF>),
}

impl ValueTrait for Value {
    type Lang = SystemF;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<SystemF>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_tylambda(self) -> Option<TyLambda<SystemF>> {
        if let Self::TyLambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }
}
