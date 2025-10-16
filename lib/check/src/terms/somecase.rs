use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::SomeCase, types::TypeGroup};

impl<Lang> Typecheck for SomeCase<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;

        let option = bound_ty.into_optional()?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), Rc::unwrap_or_clone(option.ty));
        let some_res = self.some_term.check(some_env.clone())?;
        let some_ty = some_res.ret_ty().normalize(some_env.clone());
        let some_knd = some_ty.check_kind(some_env)?;

        let none_res = self.none_term.check(env.clone())?;
        let none_ty = none_res.ret_ty().normalize(env.clone());
        let none_knd = none_ty.check_kind(env.clone())?;

        some_knd.check_equal(&none_knd)?;
        some_ty.check_equal(&none_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), some_ty);
        let deriv = TypingDerivation::somecase(conc, bound_res, some_res, none_res);
        Ok(deriv.into())
    }
}
