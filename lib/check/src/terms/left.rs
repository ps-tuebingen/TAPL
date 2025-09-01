use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Left, types::TypeGroup};

impl<Lang> Typecheck for Left<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let left_res = self.left_term.check(env.clone())?;
        let left_ty = left_res.ret_ty().normalize(env.clone());
        let left_knd = left_ty.check_kind(env.clone())?;
        let ty_norm = self.ty.clone().normalize(env.clone());
        let sum_ty = ty_norm.into_sum()?;
        let sum_kind = sum_ty.check_kind(env.clone())?;
        left_knd.check_equal(&sum_kind)?;
        sum_ty.left.check_equal(&left_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::left(conc, left_res);
        Ok(deriv.into())
    }
}
