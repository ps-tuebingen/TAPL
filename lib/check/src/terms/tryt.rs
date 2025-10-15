use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Try, types::TypeGroup};

impl<Lang> Typecheck for Try<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let term_res = self.term.check(env.clone())?;

        let term_ty = if features.normalizing {
            term_res.ret_ty().normalize(env.clone())
        } else {
            term_res.ret_ty()
        };

        let handler_res = self.handler.check(env.clone())?;
        let handler_ty = handler_res.ret_ty().normalize(env.clone());
        term_ty.check_equal(&handler_ty)?;

        if features.kinded {
            let term_knd = term_ty.check_kind(env.clone())?;
            let handler_knd = handler_ty.check_kind(env.clone())?;
            term_knd.check_equal(&handler_knd)?;
        }

        let conc = TypingConclusion::new(env, self.clone(), term_ty);
        let deriv = TypingDerivation::tryt(conc, term_res, handler_res);
        Ok(deriv.into())
    }
}
