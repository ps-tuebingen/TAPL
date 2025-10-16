use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::Fst, types::TypeGroup};

impl<Lang> Typecheck for Fst<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Kindcheck<Lang = Lang> + Normalize<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let prod = term_ty.into_product()?;

        let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(prod.fst));
        let deriv = TypingDerivation::fst(conc, term_res);
        Ok(deriv.into())
    }
}
