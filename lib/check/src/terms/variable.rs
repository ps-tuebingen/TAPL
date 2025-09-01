use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Variable};

impl<Lang> Typecheck for Variable<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let ty = env.get_var(&self.var)?;
        let conc = TypingConclusion::new(env.clone(), self.clone(), ty);
        let deriv = TypingDerivation::var(conc);
        Ok(deriv.into())
    }
}
