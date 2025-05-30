use crate::{
    errors::{Error, TypeKind},
    types::{
        Bool, Bot, Exists, ExistsBounded, Forall, ForallBounded, Fun, List, Mu, Nat, OpApp,
        OpLambda, OpLambdaSub, Optional, Product, Record, Reference, Sink, Source, Sum, Top, Tuple,
        Type, TypeVariable, Unit, Variant,
    },
};

pub trait TypeGroup: Type {
    fn check_equal(&self, other: &Self) -> Result<(), Error> {
        if self == other {
            Ok(())
        } else {
            Err(Error::ty(self, other.knd()))
        }
    }

    fn into_variable(self) -> Result<TypeVariable<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Variable))
    }

    fn into_fun(self) -> Result<Fun<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Function))
    }

    fn into_forall(self) -> Result<Forall<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Universal))
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Universal))
    }

    fn into_product(self) -> Result<Product<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Product))
    }

    fn into_tuple(self) -> Result<Tuple<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Tuple))
    }

    fn into_record(self) -> Result<Record<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Record))
    }

    fn into_variant(self) -> Result<Variant<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Variant))
    }

    fn into_sum(self) -> Result<Sum<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Sum))
    }

    fn into_optional(self) -> Result<Optional<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Option))
    }

    fn into_list(self) -> Result<List<Self>, Error> {
        Err(Error::ty(&self, TypeKind::List))
    }

    fn into_ref(self) -> Result<Reference<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Reference))
    }

    fn into_source(self) -> Result<Source<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Source))
    }

    fn into_sink(self) -> Result<Sink<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Sink))
    }

    fn into_exists(self) -> Result<Exists<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Existential))
    }
    fn into_exists_bounded(self) -> Result<ExistsBounded<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Existential))
    }

    fn into_mu(self) -> Result<Mu<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Mu))
    }

    fn into_oplambda(self) -> Result<OpLambda<Self>, Error> {
        Err(Error::ty(self, TypeKind::OpLambda))
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<Self>, Error> {
        Err(Error::ty(&self, TypeKind::OpLambda))
    }

    fn into_opapp(self) -> Result<OpApp<Self>, Error> {
        Err(Error::ty(&self, TypeKind::OpLambda))
    }

    fn into_nat(self) -> Result<Nat<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Nat))
    }

    fn into_bool(self) -> Result<Bool<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Bool))
    }

    fn into_unit(self) -> Result<Unit<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Unit))
    }

    fn into_top(self) -> Result<Top<Self>, Error> {
        Err(Error::ty(&self, TypeKind::Top))
    }

    fn into_bot(self) -> Result<Bot, Error> {
        Err(Error::ty(&self, TypeKind::Bot))
    }
}
