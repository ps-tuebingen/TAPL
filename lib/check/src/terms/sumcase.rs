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

        let bound_sum = bound_norm.into_sum()?;

        let mut left_env = env.clone();
        left_env.add_var(self.left_var.clone(), Rc::unwrap_or_clone(bound_sum.left));
        let left_res = self.left_term.check(left_env.clone())?;
        let left_ty = left_res.ret_ty();
        premises.push(left_res);

        let left_norm;
        if features.normalizing {
            let left_norm_deriv = left_ty.normalize(left_env.clone());
            left_norm = left_norm_deriv.ret_ty();
            premises.push(left_norm_deriv);
        } else {
            left_norm = left_ty;
        }

        env.add_var(self.right_var.clone(), Rc::unwrap_or_clone(bound_sum.right));
        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ret_ty();
        premises.push(right_res);

        let right_norm;
        if features.normalizing {
            let right_norm_deriv = right_ty.normalize(env.clone());
            right_norm = right_norm_deriv.ret_ty();
            premises.push(right_norm_deriv);
        } else {
            right_norm = right_ty;
        }

        if features.kinded {
            let left_knd = left_norm.check_kind(left_env)?;
            let right_knd = right_norm.check_kind(env.clone())?;
            left_knd.check_equal(&right_knd)?;
        }

        left_norm.check_equal(&right_norm)?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), right_norm);
        let deriv = TypingDerivation::sumcase(conc, premises);
        Ok(deriv.into())
    }
}
