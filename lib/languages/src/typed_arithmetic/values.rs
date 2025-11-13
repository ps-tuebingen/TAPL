use super::{TypedArithmetic, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{False, Num, True, Value as ValueTrait, ValueGroup};

#[derive(
    IntoTerm, GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq,
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
    fn into_true(self) -> Result<True<TypedArithmetic>, ValueMismatch> {
        if let Self::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }
    fn into_false(self) -> Result<False<TypedArithmetic>, ValueMismatch> {
        if let Self::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
    fn into_num(self) -> Result<Num<TypedArithmetic>, ValueMismatch> {
        if let Self::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }
}
