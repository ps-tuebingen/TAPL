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
        + Normalize<Self, Env = <Self as Subtypecheck<Self>>::Env>,
{
}
