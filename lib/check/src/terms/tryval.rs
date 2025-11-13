use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
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
        let features = Lang::features();
        let mut premises = vec![];

        let t_res = self.term.check(env.clone())?;
        let t_ty = t_res.ret_ty();
        premises.push(t_res);

        let t_norm;
        if features.normalizing() {
            let t_norm_deriv = t_ty.normalize(env.clone());
            t_norm = t_norm_deriv.ret_ty();
            premises.push(t_norm_deriv);
        } else {
            t_norm = t_ty;
        }

        let handler_res = self.handler.check(env.clone())?;
        let handler_ty = handler_res.ret_ty();
        premises.push(handler_res);

        let handler_norm;
        if features.normalizing() {
            let handler_norm_deriv = handler_ty.normalize(env.clone());
            handler_norm = handler_norm_deriv.ret_ty();
            premises.push(handler_norm_deriv);
        } else {
            handler_norm = handler_ty;
        }

        if features.kinded() {
            let t_res = t_norm.check_kind(env.clone())?.into_kind()?;
            let handler_res = handler_norm.check_kind(env.clone())?.into_kind()?;
            t_res.ret_kind().check_equal(&handler_res.ret_kind())?;
            premises.push(t_res.into());
            premises.push(handler_res.into());
        }
        let fun: Fun<Lang> = handler_norm.into_fun()?;
        fun.to.check_equal(&t_norm)?;

        let conc = TypingConclusion::new(env, self.clone(), t_norm);
        let deriv = TypingDerivation::try_val(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_tryt(true)])
    }
}
