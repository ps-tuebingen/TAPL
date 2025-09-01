use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::Nil,
    types::{List, Type},
};

impl<Lang> Typecheck for Nil<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    List<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        ty_norm.check_kind(env.clone())?.into_star()?;

        let conc =
            TypingConclusion::new(env.clone(), self.clone(), List::new(ty_norm.clone()).into());
        let deriv = TypingDerivation::nil(conc);
        Ok(deriv.into())
    }
}
