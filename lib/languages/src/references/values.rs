use super::{References, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{False, Lambda, Loc, Num, True, Unit, Value as ValueTrait, ValueGroup};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    FromVariants,
    LatexFmt,
    LangDisplay,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(References)]
pub enum Value {
    Lambda(Lambda<References>),
    Unit(Unit<References>),
    Num(Num<References>),
    Loc(Loc<References>),
    True(True<References>),
    False(False<References>),
}

impl ValueTrait for Value {
    type Lang = References;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<References>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<References>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_loc(self) -> Option<Loc<References>> {
        if let Self::Loc(loc) = self {
            Some(loc)
        } else {
            None
        }
    }

    fn into_true(self) -> Option<True<References>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<References>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }
}
