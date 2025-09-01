use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::IsNil,
    types::{Bool, List, TypeGroup},
};

impl<Lang> Typecheck for IsNil<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    List<Lang>: Into<Lang::Type>,
    Bool<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        term_ty.into_list()?;

        let conc = TypingConclusion::new(env, self.clone(), Bool::new());
        let deriv = TypingDerivation::isnil(conc, term_res);
        Ok(deriv.into())
    }
}
