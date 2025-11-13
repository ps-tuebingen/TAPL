use crate::{
    language::Language,
    types::{
        Bool, Bot, Exists, ExistsBounded, Forall, ForallBounded, Fun, List, Mu, Nat, OpApp,
        OpLambda, OpLambdaSub, Optional, Product, Record, Reference, Sink, Source, Sum, Top, Tuple,
        Type, TypeVariable, Unit, Variant,
    },
};
use errors::TypeMismatch;

/// Trait for type enums in a language
/// needed to have `into_T` functions
/// each one returns an error by default, languages overwrite ones for types of the language
pub trait TypeGroup: Type {
    /// the language these types are of
    type Lang: Language;
    /// ensure type equality
    /// # Errors
    /// returns an error if `self != other`
    fn check_equal(&self, other: &Self) -> Result<(), TypeMismatch> {
        if self == other {
            Ok(())
        } else {
            Err(TypeMismatch::new(self.to_string(), other.to_string()))
        }
    }

    /// Turn `Self` into [`TypeVariable`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_variable(self) -> Result<TypeVariable<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
    }
    /// Turn `Self` into [`Fun`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_fun(self) -> Result<Fun<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
    }
    /// Turn `Self` into [`Forall`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_forall(self) -> Result<Forall<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
    }
    /// Turn `Self` into [`ForallBounded`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_forall_bounded(self) -> Result<ForallBounded<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
    }
    /// Turn `Self` into [`Product`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_product(self) -> Result<Product<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Product".to_owned()))
    }
    /// Turn `Self` into [`Tuple`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_tuple(self) -> Result<Tuple<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Tuple".to_owned()))
    }
    /// Turn `Self` into [`Record`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_record(self) -> Result<Record<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
    }
    /// Turn `Self` into [`Variant`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_variant(self) -> Result<Variant<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
    }
    /// Turn `Self` into [`Sum`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_sum(self) -> Result<Sum<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Sum".to_owned()))
    }
    /// Turn `Self` into [`Optional`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_optional(self) -> Result<Optional<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Option".to_owned()))
    }
    /// Turn `Self` into [`List`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_list(self) -> Result<List<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
    }
    /// Turn `Self` into [`Reference`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_ref(self) -> Result<Reference<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Reference".to_owned()))
    }
    /// Turn `Self` into [`Source`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_source(self) -> Result<Source<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Source".to_owned()))
    }
    /// Turn `Self` into [`Sink`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_sink(self) -> Result<Sink<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Sink".to_owned()))
    }
    /// Turn `Self` into [`Exists`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_exists(self) -> Result<Exists<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(
            self.to_string(),
            "Existential".to_owned(),
        ))
    }
    /// Turn `Self` into [`ExistsBounded`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_exists_bounded(self) -> Result<ExistsBounded<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(
            self.to_string(),
            "Existential".to_owned(),
        ))
    }
    /// Turn `Self` into [`Mu`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_mu(self) -> Result<Mu<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Mu".to_owned()))
    }
    /// Turn `Self` into [`OpLambda`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_oplambda(self) -> Result<OpLambda<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
    }
    /// Turn `Self` into [`OpLambdaSub`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_oplambdasub(self) -> Result<OpLambdaSub<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
    }
    /// Turn `Self` into [`OpApp`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_opapp(self) -> Result<OpApp<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
    }
    /// Turn `Self` into [`Nat`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_nat(self) -> Result<Nat<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
    }

    /// Turn `Self` into [`Bool`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_bool(self) -> Result<Bool<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
    }
    /// Turn `Self` into [`Unit`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_unit(self) -> Result<Unit<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
    }

    /// Turn `Self` into [`Top`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_top(self) -> Result<Top<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
    }

    /// Turn `Self` into [`Bot`]
    /// # Errors
    /// returns an error if `Self` is a different Type
    fn into_bot(self) -> Result<Bot<Self::Lang>, TypeMismatch> {
        Err(TypeMismatch::new(self.to_string(), "Bot".to_owned()))
    }
}
