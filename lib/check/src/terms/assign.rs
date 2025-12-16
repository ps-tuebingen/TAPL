use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
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
            let lhs_knd = lhs_res.ret_kind();
            lhs_knd.clone().into_star().ok_or_else(|| {
                KindMismatch::new(lhs_knd.to_string(), "Star Kind".to_string(), self.span)
            })?;
            premises.push(lhs_res.into());
        }
        let lhs_ref = lhs_norm.clone().into_ref().ok_or_else(|| {
            TypeMismatch::new(
                lhs_norm.to_string(),
                "Reference Type".to_string(),
                self.span,
            )
        })?;

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
            let rhs_knd = rhs_res.ret_kind();
            rhs_knd.clone().into_star().ok_or_else(|| {
                KindMismatch::new(rhs_knd.to_string(), "Star Kind".to_string(), self.span)
            })?;
            premises.push(rhs_res.into());
        }

        if *lhs_ref.ty != rhs_norm {
            return Err(
                TypeMismatch::new(lhs_ref.to_string(), rhs_norm.to_string(), self.span).into(),
            );
        }

        let conc = TypingConclusion::new(env, self.clone(), UnitTy::<Lang>::new(self.span));
        let deriv = TypingDerivation::assign(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_assign()])
    }
}
