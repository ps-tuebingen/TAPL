use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind, terms::Term, types::Type};

pub mod definition;
pub mod errors;
pub mod program;
pub mod terms;
pub mod types;
pub mod untyped;
use errors::CheckError;

pub trait Typecheck {
    type Type: Type;
    type Term: Term;
    type Deriv: Derivation<Self::Term, Self::Type>;

    fn check_start(&self) -> Result<Self::Deriv, CheckError> {
        self.check(Environment::default())
    }

    fn check(&self, env: Environment<Self::Type>) -> Result<Self::Deriv, CheckError>;
}

pub trait Subtypecheck<Ty>
where
    Self: Type,
    Ty: Type,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError>;
}

pub trait Kindcheck<Ty>
where
    Ty: Type,
{
    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, CheckError>;
}

pub trait Normalize<Ty>
where
    Ty: Type,
{
    fn normalize(self, env: Environment<Ty>) -> Ty;
}
