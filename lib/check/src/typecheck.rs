use derivations::Derivation;
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language};

pub trait Typecheck {
    type Lang: Language;

    fn check_start(&self) -> Result<Derivation<Self::Lang>, CheckError> {
        self.check(Environment::default())
    }

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError>;
}
