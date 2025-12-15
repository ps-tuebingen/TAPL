use super::{LambdaOmega, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{False, Lambda, Num, True, TyLambda, Unit, Value as ValueTrait, ValueGroup};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    FromVariants,
    LatexFmt,
    LangDisplay,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[Lang(LambdaOmega)]
pub enum Value {
    Unit(Unit<LambdaOmega>),
    True(True<LambdaOmega>),
    False(False<LambdaOmega>),
    Num(Num<LambdaOmega>),
    Lambda(Lambda<LambdaOmega>),
    TyLambda(TyLambda<LambdaOmega>),
}

impl ValueTrait for Value {
    type Lang = LambdaOmega;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Option<True<LambdaOmega>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<LambdaOmega>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<LambdaOmega>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_lambda(self) -> Option<Lambda<LambdaOmega>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_tylambda(self) -> Option<TyLambda<LambdaOmega>> {
        if let Self::TyLambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }
}
