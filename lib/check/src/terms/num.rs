use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Num, types::Nat};

impl<Lang> Typecheck for Num<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Nat<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let conc = TypingConclusion::new(env.clone(), self.clone(), Nat::new());
        Ok(TypingDerivation::num(conc).into())
    }
}
