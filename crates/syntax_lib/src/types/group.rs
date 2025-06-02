use crate::types::{
    Bool, Bot, Exists, ExistsBounded, Forall, ForallBounded, Fun, List, Mu, Nat, OpApp, OpLambda,
    OpLambdaSub, Optional, Product, Record, Reference, Sink, Source, Sum, Top, Tuple, Type,
    TypeVariable, Unit, Variant,
};
use common::errors::{TypeKind, TypeMismatch};

pub trait TypeGroup: Type {
    fn check_equal(&self, other: &Self) -> Result<(), TypeMismatch> {
        if self == other {
            Ok(())
        } else {
            Err(TypeMismatch::new(self.knd(), other.knd()))
        }
    }

    fn into_variable(self) -> Result<TypeVariable<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Variable))
    }

    fn into_fun(self) -> Result<Fun<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Function))
    }

    fn into_forall(self) -> Result<Forall<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Universal))
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Universal))
    }

    fn into_product(self) -> Result<Product<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Product))
    }

    fn into_tuple(self) -> Result<Tuple<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Tuple))
    }

    fn into_record(self) -> Result<Record<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Record))
    }

    fn into_variant(self) -> Result<Variant<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Variant))
    }

    fn into_sum(self) -> Result<Sum<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Sum))
    }

    fn into_optional(self) -> Result<Optional<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Option))
    }

    fn into_list(self) -> Result<List<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::List))
    }

    fn into_ref(self) -> Result<Reference<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Reference))
    }

    fn into_source(self) -> Result<Source<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Source))
    }

    fn into_sink(self) -> Result<Sink<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Sink))
    }

    fn into_exists(self) -> Result<Exists<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Existential))
    }
    fn into_exists_bounded(self) -> Result<ExistsBounded<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Existential))
    }

    fn into_mu(self) -> Result<Mu<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Mu))
    }

    fn into_oplambda(self) -> Result<OpLambda<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::OpLambda))
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::OpLambda))
    }

    fn into_opapp(self) -> Result<OpApp<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::OpLambda))
    }

    fn into_nat(self) -> Result<Nat<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
    }

    fn into_bool(self) -> Result<Bool<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Bool))
    }

    fn into_unit(self) -> Result<Unit<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Unit))
    }

    fn into_top(self) -> Result<Top<Self>, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Top))
    }

    fn into_bot(self) -> Result<Bot, TypeMismatch> {
        Err(TypeMismatch::new(self.knd(), TypeKind::Bot))
    }
}
