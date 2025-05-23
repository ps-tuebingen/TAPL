use common::errors::{Error, ErrorKind, ErrorLocation};
use syntax::{kinds::Kind, types::Type};

pub mod env;
pub mod types;
pub mod untyped;

pub use env::CheckEnvironment;

pub trait Typecheck {
    type Type: Type;
    type Env: CheckEnvironment<Type = Self::Type>;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Self::Env::default())
    }

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error>;
}

pub trait Subtypecheck<Ty>
where
    Self: Type,
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error>;
}

pub trait Kindcheck<Ty>
where
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error>;
}

pub trait Normalize<Ty>
where
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;
    fn normalize(self, env: &mut Self::Env) -> Ty;
}

pub fn to_check_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Check,
    }
}

pub fn to_subty_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Subtyping,
    }
}

pub fn to_kind_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Kind,
    }
}
