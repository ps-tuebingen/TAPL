use super::{Exceptions, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{
    Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    LatexFmt,
    FromVariants,
    LangDisplay,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(Exceptions)]
pub enum Value {
    Lambda(Lambda<Exceptions>),
    Num(Num<Exceptions>),
    Unit(Unit<Exceptions>),
    True(True<Exceptions>),
    False(False<Exceptions>),
    Exception(Exception<Exceptions>),
    Raise(Raise<Exceptions>),
}

impl ValueTrait for Value {
    type Lang = Exceptions;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Option<True<Exceptions>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<Exceptions>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }

    fn into_exception(self) -> Option<Exception<Exceptions>> {
        if let Self::Exception(ex) = self {
            Some(ex)
        } else {
            None
        }
    }

    fn into_lambda(self) -> Option<Lambda<Exceptions>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_raise(self) -> Option<Raise<Exceptions>> {
        if let Self::Raise(raise) = self {
            Some(raise)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<Exceptions>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }
}
