use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Right, types::TypeGroup};

impl<Lang> Typecheck for Right<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ret_ty();
        premises.push(right_res);

        let right_norm;
        let sum_norm;
        if features.normalizing {
            let right_norm_deriv = right_ty.normalize(env.clone());
            let sum_norm_deriv = self.ty.clone().normalize(env.clone());
            sum_norm = sum_norm_deriv.ret_ty();
            right_norm = right_norm_deriv.ret_ty();
            premises.push(right_norm_deriv);
            premises.push(sum_norm_deriv);
        } else {
            right_norm = right_ty;
            sum_norm = self.ty.clone();
        }

        let sum_ty = sum_norm.into_sum()?;
        sum_ty.right.check_equal(&right_norm)?;

        if features.kinded {
            let right_knd = right_norm.check_kind(env.clone())?;
            let sum_knd = sum_ty.check_kind(env.clone())?;
            right_knd.check_equal(&sum_knd)?;
        }

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::right(conc, premises);

        Ok(deriv.into())
    }
}
