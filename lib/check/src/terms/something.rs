use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Something, types::Optional};

impl<Lang> Typecheck for Something<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Optional<Lang>: Into<Lang::Type>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let conc = TypingConclusion::new(env, self.clone(), Optional::new(term_ty.clone()));
        let deriv = TypingDerivation::something(conc, term_res);
        Ok(deriv.into())
    }
}
