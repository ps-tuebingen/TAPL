use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::Deref, types::TypeGroup};

impl<Lang> Typecheck for Deref<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let ref_ty = term_ty.into_ref()?;

        let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(ref_ty.ty));
        let deriv = TypingDerivation::deref(conc, term_res);

        Ok(deriv.into())
    }
}
