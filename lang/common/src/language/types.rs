use crate::{
    errors::ErrorKind,
    subst::SubstType,
    types::{Bool, Forall, ForallBounded, Fun, Nat, Type},
};

pub trait LanguageType
where
    Self: Type + SubstType<Self, Target = Self>,
{
    fn into_fun(self) -> Result<Fun<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Function Type".to_owned(),
        })
    }

    fn into_forall(self) -> Result<Forall<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Universal Type".to_owned(),
        })
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Universal Type".to_owned(),
        })
    }

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
