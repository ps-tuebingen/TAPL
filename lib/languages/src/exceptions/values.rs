use super::{Exceptions, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{
    Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(
    IntoTerm, GrammarDescribe, LatexFmt, FromVariants, LangDisplay, Debug, Clone, PartialEq, Eq,
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
    fn into_true(self) -> Result<True<Exceptions>, ValueMismatch> {
        if let Self::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Exceptions>, ValueMismatch> {
        if let Self::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_exception(self) -> Result<Exception<Exceptions>, ValueMismatch> {
        if let Self::Exception(ex) = self {
            Ok(ex)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Exception".to_owned()))
        }
    }

    fn into_lambda(self) -> Result<Lambda<Exceptions>, ValueMismatch> {
        if let Self::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_raise(self) -> Result<Raise<Exceptions>, ValueMismatch> {
        if let Self::Raise(raise) = self {
            Ok(raise)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Raise".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Exceptions>, ValueMismatch> {
        if let Self::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }
}
