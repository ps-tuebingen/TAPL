use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::SumCase, types::TypeGroup};

impl<Lang> Typecheck for SumCase<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;
        let bound_sum = bound_ty.into_sum()?;

        let mut left_env = env.clone();
        left_env.add_var(self.left_var.clone(), Rc::unwrap_or_clone(bound_sum.left));
        let left_res = self.left_term.check(left_env.clone())?;
        let left_ty = left_res.ret_ty().normalize(left_env.clone());
        let left_knd = left_ty.check_kind(left_env)?;

        env.add_var(self.right_var.clone(), Rc::unwrap_or_clone(bound_sum.right));
        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ret_ty().normalize(env.clone());
        let right_knd = right_ty.check_kind(env.clone())?;

        left_knd.check_equal(&right_knd)?;
        left_ty.check_equal(&right_ty)?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), right_ty);
        let deriv = TypingDerivation::sumcase(conc, bound_res, left_res, right_res);
        Ok(deriv.into())
    }
}
