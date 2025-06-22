use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind, terms::Term, types::Type};

pub mod errors;
pub mod terms;
pub mod types;
pub mod untyped;
use errors::CheckError;

pub trait Typecheck {
    type Type: Type;
    type Term: Term;

    fn check_start(&self) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        self.check(Environment::default())
    }

    fn check(
        &self,
        env: Environment<Self::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>>;
}

pub trait Subtypecheck<Ty>
where
    Self: Type,
    Ty: Type,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError<Ty>>;
}

pub trait Kindcheck<Ty>
where
    Ty: Type,
{
    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, CheckError<Ty>>;
}

pub trait Normalize<Ty>
where
    Ty: Type,
{
    fn normalize(self, env: Environment<Ty>) -> Ty;
}
