use crate::{
    check::{Kindcheck, Subtypecheck},
    errors::ErrorKind,
    eval::Normalize,
    subst::SubstType,
    types::{
        Bool, Bot, Exists, ExistsBounded, Forall, ForallBounded, Fun, List, Mu, Nat, OpApp,
        OpLambda, OpLambdaSub, Optional, Product, Record, Reference, Sink, Source, Sum, Top, Tuple,
        Type, TypeVariable, Unit, Variant,
    },
};

pub trait LanguageType
where
    Self: Type
        + SubstType<Self, Target = Self>
        + Subtypecheck<Self>
        + Kindcheck<Self>
        + Normalize<Self>,
{
    fn check_equal(&self, other: &Self) -> Result<(), ErrorKind> {
        let self_norm = self.clone().normalize();
        let other_norm = other.clone().normalize();
        if self_norm == other_norm {
            Ok(())
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: other.to_string(),
            })
        }
    }

    fn into_variable(self) -> Result<TypeVariable<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Type Varirable".to_owned(),
        })
    }

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

    fn into_ref(self) -> Result<Reference<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Reference".to_owned(),
        })
    }

    fn into_source(self) -> Result<Source<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Source".to_owned(),
        })
    }

    fn into_sink(self) -> Result<Sink<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Sink".to_owned(),
        })
    }

    fn into_exists(self) -> Result<Exists<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Existential Type".to_owned(),
        })
    }
    fn into_exists_bounded(self) -> Result<ExistsBounded<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Existential Type".to_owned(),
        })
    }

    fn into_mu(self) -> Result<Mu<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Mu".to_owned(),
        })
    }

    fn into_oplambda(self) -> Result<OpLambda<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Operator Abstraction".to_owned(),
        })
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Operator Abstraction".to_owned(),
        })
    }

    fn into_opapp(self) -> Result<OpApp<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Operator Application".to_owned(),
        })
    }

    fn into_nat(self) -> Result<Nat<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Nat".to_owned(),
        })
    }

    fn into_bool(self) -> Result<Bool<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Bool".to_owned(),
        })
    }

    fn into_unit(self) -> Result<Unit<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Unit".to_owned(),
        })
    }

    fn into_top(self) -> Result<Top<Self>, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Top".to_owned(),
        })
    }

    fn into_bot(self) -> Result<Bot, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Bot".to_owned(),
        })
    }
}
