use crate::{
    errors::ErrorKind,
    subst::SubstType,
    types::{
        Bool, Forall, ForallBounded, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Type,
        Variant,
    },
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

    fn into_product(self) -> Result<Product<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Product Type".to_owned(),
        })
    }

    fn into_tuple(self) -> Result<Tuple<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Tuple".to_owned(),
        })
    }

    fn into_record(self) -> Result<Record<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Record".to_owned(),
        })
    }

    fn into_variant(self) -> Result<Variant<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Variant".to_owned(),
        })
    }

    fn into_sum(self) -> Result<Sum<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Sum".to_owned(),
        })
    }

    fn into_optional(self) -> Result<Optional<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Option".to_owned(),
        })
    }

    fn into_list(self) -> Result<List<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "List".to_owned(),
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
