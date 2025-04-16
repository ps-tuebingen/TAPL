use crate::{
    errors::ErrorKind,
    subst::SubstType,
    types::{Bool, Fun, Nat, Type},
};

pub trait LanguageType
where
    Self: Type + SubstType<Self, Target = Self>,
{
    fn into_fun<Ty>(self) -> Result<Fun<Ty>, ErrorKind>
    where
        Ty: Type;

    fn into_nat(self) -> Result<Nat, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Nat".to_owned(),
        })
    }

    fn into_bool(self) -> Result<Bool, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Bool".to_owned(),
        })
    }

    fn check_equal(&self, other: &Self) -> Result<(), ErrorKind> {
        if *self == *other {
            Ok(())
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: other.to_string(),
            })
        }
    }
}
