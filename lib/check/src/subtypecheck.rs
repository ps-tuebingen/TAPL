use derivations::Derivation;
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language};

pub trait Subtypecheck {
    type Lang: Language;

    fn check_subtype(
        &self,
        sup: &<Self::Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError>;
}
