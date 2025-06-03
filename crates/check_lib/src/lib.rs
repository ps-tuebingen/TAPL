use syntax::{env::Environment, kinds::Kind, types::Type};

pub mod errors;
pub mod terms;
pub mod types;
pub mod untyped;

pub trait Typecheck {
    type CheckError: std::error::Error;
    type Type: Type;

    fn check_start(&self) -> Result<Self::Type, Self::CheckError> {
        self.check(&mut Environment::default())
    }

    fn check(&self, env: &mut Environment<Self::Type>) -> Result<Self::Type, Self::CheckError>;
}

pub trait Subtypecheck<Ty>
where
    Self: Type,
    Ty: Type,
{
    type CheckError: std::error::Error;

    fn check_subtype(&self, sup: &Ty, env: &mut Environment<Ty>) -> Result<(), Self::CheckError>;
}

pub trait Kindcheck<Ty>
where
    Ty: Type,
{
    type CheckError: std::error::Error;

    fn check_kind(&self, env: &mut Environment<Ty>) -> Result<Kind, Self::CheckError>;
}

pub trait Normalize<Ty>
where
    Ty: Type,
{
    fn normalize(self, env: &mut Environment<Ty>) -> Ty;
}
