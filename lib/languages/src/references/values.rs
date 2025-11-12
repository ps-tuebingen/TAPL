use super::{References, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{False, Lambda, Loc, Num, True, Unit, Value as ValueTrait, ValueGroup};

#[derive(
    IntoTerm, GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq,
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
    fn into_lambda(self) -> Result<Lambda<References>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<References>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_loc(self) -> Result<Loc<References>, ValueMismatch> {
        if let Value::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Location".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<References>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<References>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
}
