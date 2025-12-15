use super::{TypedArithmetic, terms::Term};
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
#[Lang(TypedArithmetic)]
pub enum Value {
    True(True<TypedArithmetic>),
    False(False<TypedArithmetic>),
    Num(Num<TypedArithmetic>),
}

impl ValueTrait for Value {
    type Lang = TypedArithmetic;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Option<True<TypedArithmetic>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }
    fn into_false(self) -> Option<False<TypedArithmetic>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }
    fn into_num(self) -> Option<Num<TypedArithmetic>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }
}
