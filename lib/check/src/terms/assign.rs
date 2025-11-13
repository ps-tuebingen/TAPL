use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
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
        let features = Lang::features();
        let mut premises = vec![];

        let lhs_res = self.lhs.check(env.clone())?;
        let lhs_ty = lhs_res.ret_ty();
        premises.push(lhs_res);

        let lhs_norm;
        if features.normalizing() {
            let lhs_norm_deriv = lhs_ty.normalize(env.clone());
            lhs_norm = lhs_norm_deriv.ret_ty();
            premises.push(lhs_norm_deriv);
        } else {
            lhs_norm = lhs_ty;
        }

        if features.kinded() {
            let lhs_res = lhs_norm.check_kind(env.clone())?.into_kind()?;
            lhs_res.ret_kind().into_star()?;
            premises.push(lhs_res.into());
        }
        let lhs_ref = lhs_norm.into_ref()?;

        let rhs_res = self.rhs.check(env.clone())?;
        let rhs_ty = rhs_res.ret_ty();

        let rhs_norm;
        if features.normalizing() {
            let rhs_norm_deriv = rhs_ty.normalize(env.clone());
            rhs_norm = rhs_norm_deriv.ret_ty();
            premises.push(rhs_norm_deriv);
        } else {
            rhs_norm = rhs_ty;
        }
        if features.kinded() {
            let rhs_res = rhs_norm.check_kind(env.clone())?.into_kind()?;
            rhs_res.ret_kind().into_star()?;
            premises.push(rhs_res.into());
        }
        lhs_ref.ty.check_equal(&rhs_norm)?;

        let conc = TypingConclusion::new(env, self.clone(), UnitTy::<Lang>::new());
        let deriv = TypingDerivation::assign(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_assign()])
    }
}
