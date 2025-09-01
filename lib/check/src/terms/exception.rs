use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Exception, types::Type};

impl<Lang> Typecheck for Exception<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        ty_norm.check_kind(env.clone())?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), ty_norm);
        let deriv = TypingDerivation::exception(conc);
        Ok(deriv.into())
    }
}
