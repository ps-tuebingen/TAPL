use super::terms::LanguageTerm;
use crate::{
    errors::ErrorKind,
    values::{False, Lambda, Num, Raise, True, Value},
};

pub trait LanguageValue
where
    Self: Value + Into<<Self as LanguageValue>::Term>,
{
    type Term: LanguageTerm;

    fn into_lambda(self) -> Result<Lambda<<Self as LanguageValue>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise(self) -> Result<Raise<<Self as LanguageValue>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }

    fn into_true(self) -> Result<True<<Self as LanguageValue>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "True".to_owned(),
        })
    }

    fn into_false(self) -> Result<False<<Self as LanguageValue>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "False".to_owned(),
        })
    }

    fn into_num(self) -> Result<Num<<Self as LanguageValue>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Number".to_owned(),
        })
    }
}
