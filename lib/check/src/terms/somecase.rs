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
        let features = Lang::features();
        let mut premises = vec![];

        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty();
        premises.push(bound_res);

        let bound_norm;
        if features.normalizing {
            let bound_norm_deriv = bound_ty.normalize(env.clone());
            bound_norm = bound_norm_deriv.ret_ty();
            premises.push(bound_norm_deriv);
        } else {
            bound_norm = bound_ty;
        }

        if features.kinded {
            bound_norm.check_kind(env.clone())?.into_star()?;
        }

        let option = bound_norm.into_optional()?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), Rc::unwrap_or_clone(option.ty));
        let some_res = self.some_term.check(some_env.clone())?;
        let some_ty = some_res.ret_ty();
        premises.push(some_res);

        let some_norm;
        if features.normalizing {
            let some_norm_deriv = some_ty.normalize(some_env.clone());
            some_norm = some_norm_deriv.ret_ty();
            premises.push(some_norm_deriv);
        } else {
            some_norm = some_ty;
        }

        let none_res = self.none_term.check(env.clone())?;
        let none_ty = none_res.ret_ty();
        premises.push(none_res);

        let none_norm;
        if features.normalizing {
            let none_norm_res = none_ty.normalize(env.clone());
            none_norm = none_norm_res.ret_ty();
            premises.push(none_norm_res);
        } else {
            none_norm = none_ty;
        }

        if features.kinded {
            let some_knd = some_norm.check_kind(some_env)?;
            let none_knd = none_norm.check_kind(env.clone())?;
            some_knd.check_equal(&none_knd)?;
        }

        some_norm.check_equal(&none_norm)?;

        let conc = TypingConclusion::new(env, self.clone(), some_norm);
        let deriv = TypingDerivation::somecase(conc, premises);
        Ok(deriv.into())
    }
}
