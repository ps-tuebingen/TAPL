use crate::types::{
    Bool, Bot, Exists, ExistsBounded, Forall, ForallBounded, Fun, List, Mu, Nat, OpApp, OpLambda,
    OpLambdaSub, Optional, Product, Record, Reference, Sink, Source, Sum, Top, Tuple, Type,
    TypeVariable, Unit, Variant,
};
use errors::TypeMismatch;

pub trait TypeGroup: Type {
    fn check_equal(&self, other: &Self) -> Result<(), TypeMismatch> {
        if self == other {
            Ok(())
        } else {
            Err(TypeMismatch::new(self.to_string(), other.to_string()))
        }
    }

    fn into_variable(self) -> Result<TypeVariable<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
    }

    fn into_fun(self) -> Result<Fun<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
    }

    fn into_forall(self) -> Result<Forall<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
    }

    fn into_product(self) -> Result<Product<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Product".to_owned()))
    }

    fn into_tuple(self) -> Result<Tuple<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Tuple".to_owned()))
    }

    fn into_record(self) -> Result<Record<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
    }

    fn into_variant(self) -> Result<Variant<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
    }

    fn into_sum(self) -> Result<Sum<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Sum".to_owned()))
    }

    fn into_optional(self) -> Result<Optional<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Option".to_owned()))
    }

    fn into_list(self) -> Result<List<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
    }

    fn into_ref(self) -> Result<Reference<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Reference".to_owned()))
    }

    fn into_source(self) -> Result<Source<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Source".to_owned()))
    }

    fn into_sink(self) -> Result<Sink<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Sink".to_owned()))
    }

    fn into_exists(self) -> Result<Exists<Self>, TypeMismatch> {
        Err(TypeMismatch::new(
            self.to_string(),
            "Existential".to_owned(),
        ))
    }
    fn into_exists_bounded(self) -> Result<ExistsBounded<Self>, TypeMismatch> {
        Err(TypeMismatch::new(
            self.to_string(),
            "Existential".to_owned(),
        ))
    }

    fn into_mu(self) -> Result<Mu<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Mu".to_owned()))
    }

    fn into_oplambda(self) -> Result<OpLambda<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
    }

    fn into_opapp(self) -> Result<OpApp<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
    }

    fn into_nat(self) -> Result<Nat<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
    }

    fn into_bool(self) -> Result<Bool<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
    }

    fn into_unit(self) -> Result<Unit<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
    }

    fn into_top(self) -> Result<Top<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
    }

    fn into_bot(self) -> Result<Bot<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Bot".to_owned()))
    }
}
