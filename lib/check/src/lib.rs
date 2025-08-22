use derivations::Derivation;
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, language::Language};

pub mod definition;
pub mod program;
pub mod terms;
pub mod types;
pub mod untyped;

pub trait Typecheck {
    type Lang: Language;

    fn check_start(
        &self,
    ) -> Result<
        Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
        CheckError,
    > {
        self.check(Environment::default())
    }

    fn check(
        &self,
        env: Environment<<Self::Lang as Language>::Type>,
    ) -> Result<
        Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
        CheckError,
    >;
}

pub trait Subtypecheck {
    type Lang: Language;

    fn check_subtype(
        &self,
        sup: &<Self::Lang as Language>::Type,
        env: Environment<<Self::Lang as Language>::Type>,
    ) -> Result<
        Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
        CheckError,
    >;
}

pub trait Kindcheck {
    type Lang: Language;

    fn check_kind(
        &self,
        env: Environment<<Self::Lang as Language>::Type>,
    ) -> Result<Kind, CheckError>;
}

pub trait Normalize {
    type Lang: Language;
    fn normalize(
        self,
        env: Environment<<Self::Lang as Language>::Type>,
    ) -> <Self::Lang as Language>::Type;
}
