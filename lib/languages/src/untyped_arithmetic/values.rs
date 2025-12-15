use super::{UntypedArithmetic, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{False, Num, True, Value as ValueTrait, ValueGroup};

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
#[Lang(UntypedArithmetic)]
pub enum Value {
    True(True<UntypedArithmetic>),
    False(False<UntypedArithmetic>),
    Num(Num<UntypedArithmetic>),
}

impl ValueTrait for Value {
    type Lang = UntypedArithmetic;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Option<True<UntypedArithmetic>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<UntypedArithmetic>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<UntypedArithmetic>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }
}
