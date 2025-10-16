use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, language::Language};

pub trait Kindcheck {
    type Lang: Language;

    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError>;
}
