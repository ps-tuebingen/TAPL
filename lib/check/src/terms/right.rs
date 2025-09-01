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
        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ret_ty().normalize(env.clone());
        let right_knd = right_ty.check_kind(env.clone())?;

        let sum_ty = self.ty.clone().normalize(env.clone()).into_sum()?;
        let sum_knd = sum_ty.check_kind(env.clone())?;

        right_knd.check_equal(&sum_knd)?;
        sum_ty.right.check_equal(&right_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::right(conc, right_res);

        Ok(deriv.into())
    }
}
