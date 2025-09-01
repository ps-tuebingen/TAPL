use crate::{Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::False,
    types::{Bool, Type},
};

impl<Lang> Typecheck for False<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Type + Normalize<Lang = Lang>,
    Bool<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let conc = TypingConclusion::new(env.clone(), self.clone(), Bool::new());
        Ok(TypingDerivation::fls(conc).into())
    }
}
