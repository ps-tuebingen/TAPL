use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::TryWithVal,
    types::{Fun, TypeGroup},
};

impl<Lang> Typecheck for TryWithVal<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let t_res = self.term.check(env.clone())?;
        let t_ty = t_res.ret_ty().normalize(env.clone());
        let t_knd = t_ty.check_kind(env.clone())?;

        let handler_res = self.handler.check(env.clone())?;
        let handler_ty = handler_res.ret_ty().normalize(env.clone());
        let handler_knd = handler_ty.check_kind(env.clone())?;
        let fun: Fun<Lang> = handler_ty.into_fun()?;

        t_knd.check_equal(&handler_knd)?;
        fun.to.check_equal(&t_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), t_ty);
        let deriv = TypingDerivation::try_val(conc, t_res, handler_res);
        Ok(deriv.into())
    }
}
