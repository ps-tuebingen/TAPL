use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    language::LanguageType,
    types::Type,
};

pub mod env;
pub use env::CheckEnvironment;

pub trait Typecheck {
    type Type: LanguageType;
    type Env: CheckEnvironment<Type = Self::Type>;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Self::Env::default())
    }

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error>;
}

pub trait Subtypecheck: Type {
    type Env: CheckEnvironment<Type = Self>;

    fn check_subtype(&self, sup: &Self, env: &mut Self::Env) -> Result<(), Error>;
    fn check_supertype(&self, sub: &Self, env: &mut Self::Env) -> Result<(), Error> {
        sub.check_subtype(self, env)
    }
}

pub fn to_check_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Check,
    }
}
