use syntax::{kinds::Kind, types::Type};

pub mod env;
pub mod errors;
pub mod terms;
pub mod types;
pub mod untyped;

pub use env::CheckEnvironment;

pub trait Typecheck {
    type CheckError: std::error::Error
        + From<<<Self as Typecheck>::Env as CheckEnvironment>::CheckError>;
    type Type: Type;
    type Env: CheckEnvironment<Type = Self::Type>;

    fn check_start(&self) -> Result<Self::Type, Self::CheckError> {
        self.check(&mut Self::Env::default())
    }

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError>;
}

pub trait Subtypecheck<Ty>
where
    Self: Type,
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;
    type CheckError: std::error::Error
        + From<<<Self as Subtypecheck<Ty>>::Env as CheckEnvironment>::CheckError>;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Self::CheckError>;
}

pub trait Kindcheck<Ty>
where
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;
    type CheckError: std::error::Error
        + From<<<Self as Kindcheck<Ty>>::Env as CheckEnvironment>::CheckError>;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Self::CheckError>;
}

pub trait Normalize<Ty>
where
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;
    fn normalize(self, env: &mut Self::Env) -> Ty;
}
