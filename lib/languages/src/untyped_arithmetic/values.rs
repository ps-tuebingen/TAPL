use super::{UntypedArithmetic, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{False, Num, True, Value as ValueTrait, ValueGroup};

#[derive(
    IntoTerm, GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq,
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
    fn into_true(self) -> Result<True<UntypedArithmetic>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<UntypedArithmetic>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<UntypedArithmetic>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }
}
