use crate::{
    language::Language,
    types::{
        Bool, Bot, Exists, ExistsBounded, Forall, ForallBounded, Fun, List, Mu, Nat, OpApp,
        OpLambda, OpLambdaSub, Optional, Product, Record, Reference, Sink, Source, Sum, Top, Tuple,
        Type, TypeVariable, Unit, Variant,
    },
};

/// Trait for type enums in a language
/// needed to have `into_T` functions
/// each one returns an error by default, languages overwrite ones for types of the language
pub trait TypeGroup: Type {
    /// the language these types are of
    type Lang: Language;

    /// Turn `Self` into [`TypeVariable`]

    fn into_variable(self) -> Option<TypeVariable<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Fun`]

    fn into_fun(self) -> Option<Fun<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Forall`]

    fn into_forall(self) -> Option<Forall<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`ForallBounded`]

    fn into_forall_bounded(self) -> Option<ForallBounded<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Product`]

    fn into_product(self) -> Option<Product<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Tuple`]

    fn into_tuple(self) -> Option<Tuple<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Record`]

    fn into_record(self) -> Option<Record<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Variant`]

    fn into_variant(self) -> Option<Variant<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Sum`]

    fn into_sum(self) -> Option<Sum<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Optional`]

    fn into_optional(self) -> Option<Optional<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`List`]

    fn into_list(self) -> Option<List<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Reference`]

    fn into_ref(self) -> Option<Reference<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Source`]

    fn into_source(self) -> Option<Source<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Sink`]

    fn into_sink(self) -> Option<Sink<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Exists`]

    fn into_exists(self) -> Option<Exists<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`ExistsBounded`]

    fn into_exists_bounded(self) -> Option<ExistsBounded<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Mu`]

    fn into_mu(self) -> Option<Mu<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`OpLambda`]

    fn into_oplambda(self) -> Option<OpLambda<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`OpLambdaSub`]

    fn into_oplambdasub(self) -> Option<OpLambdaSub<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`OpApp`]

    fn into_opapp(self) -> Option<OpApp<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Nat`]

    fn into_nat(self) -> Option<Nat<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`Bool`]

    fn into_bool(self) -> Option<Bool<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Unit`]

    fn into_unit(self) -> Option<Unit<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`Top`]

    fn into_top(self) -> Option<Top<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`Bot`]

    fn into_bot(self) -> Option<Bot<Self::Lang>> {
        None
    }
}
