use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::Assign,
    types::{TypeGroup, Unit as UnitTy},
};

impl<Lang> Typecheck for Assign<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    UnitTy<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let lhs_res = self.lhs.check(env.clone())?;
        let lhs_ty = lhs_res.ret_ty().normalize(env.clone());
        lhs_ty.check_kind(env.clone())?.into_star()?;
        let lhs_ref = lhs_ty.into_ref()?;

        let rhs_res = self.rhs.check(env.clone())?;
        let rhs_ty = rhs_res.ret_ty().normalize(env.clone());
        rhs_ty.check_kind(env.clone())?.into_star()?;
        lhs_ref.ty.check_equal(&rhs_ty)?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), UnitTy::<Lang>::new());
        let deriv = TypingDerivation::assign(conc, lhs_res, rhs_res);

        Ok(deriv.into())
    }
}
