use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind, terms::Term, types::Type};

pub mod errors;
pub mod terms;
pub mod types;
pub mod untyped;

pub struct CheckResult<T, Ty>
where
    T: Term,
    Ty: Type,
{
    ty: Ty,
    derivation: Derivation<T, Ty>,
}

impl<T, Ty> CheckResult<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1, derivation: Derivation<T, Ty>) -> CheckResult<T, Ty>
    where
        Ty1: Into<Ty>,
    {
        CheckResult { ty, derivation }
    }
}

pub trait Typecheck {
    type CheckError: std::error::Error;
    type Type: Type;
    type Term: Term;

    fn check_start(&self) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        self.check(&mut Environment::default())
    }

    fn check(
        &self,
        env: &mut Environment<Self::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError>;
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
