use derivations::Derivation;
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, language::Language};

pub mod definition;
pub mod program;
pub mod terms;
pub mod types;

pub trait Typecheck {
    type Lang: Language;

    fn check_start(&self) -> Result<Derivation<Self::Lang>, CheckError> {
        self.check(Environment::default())
    }

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError>;
}

pub trait Subtypecheck {
    type Lang: Language;

    fn check_subtype(
        &self,
        sup: &<Self::Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError>;
}

pub trait Kindcheck {
    type Lang: Language;

    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError>;
}

pub trait Normalize {
    type Lang: Language;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type;
}
