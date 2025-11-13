use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::{NameMismatch, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use std::rc::Rc;
use syntax::{
    env::Environment,
    language::Language,
    types::{ExistsBounded, Top, TypeGroup},
};

impl<Lang> Subtypecheck for ExistsBounded<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang> + TypeGroup<Lang = Lang> + Subtypecheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        mut env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        let mut premises = vec![];

        let sup_norm;
        let self_norm;
        if features.normalizing() {
            let sup_norm_deriv = sup.clone().normalize(env.clone());
            sup_norm = sup_norm_deriv.ret_ty();
            premises.push(sup_norm_deriv);
            let self_norm_deriv = self.sup_ty.clone().normalize(env.clone());
            self_norm = self_norm_deriv.ret_ty();
            premises.push(self_norm_deriv);
        } else {
            sup_norm = sup.clone();
            self_norm = Rc::unwrap_or_clone(self.sup_ty.clone());
        }

        let other_exists = sup_norm.into_exists_bounded()?;
        other_exists.sup_ty.check_equal(&self_norm)?;
        if self.var != other_exists.var {
            return Err(NameMismatch::new(&other_exists.var, &self.var).into());
        }
        let old_env = env.clone();
        env.add_tyvar_super(other_exists.var, Rc::unwrap_or_clone(self.sup_ty.clone()));

        let inner_res;
        if features.normalizing() {
            let inner_deriv = self.ty.clone().normalize(env.clone());
            inner_res = inner_deriv.ret_ty();
            premises.push(inner_deriv);
        } else {
            inner_res = Rc::unwrap_or_clone(self.ty.clone());
        }
        let inner_sub_deriv = inner_res.check_subtype(&(*other_exists.ty), env)?;
        premises.push(inner_sub_deriv);
        Ok(SubtypeDerivation::exists_bounded(old_env, self.clone(), sup.clone(), premises).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_top(), DerivationRule::sub_exists(true)])
    }
}

impl<Lang> Kindcheck for ExistsBounded<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let sup_res = self.sup_ty.check_kind(env.clone())?.into_kind()?;
        env.add_tyvar_kind(self.var.clone(), sup_res.ret_kind());
        let inner_res = self.ty.check_kind(env)?.into_kind()?;
        Ok(
            KindingDerivation::exists_bound(self.clone(), inner_res.ret_kind(), sup_res, inner_res)
                .into(),
        )
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_exists(true)])
    }
}

impl<Lang> Normalize for ExistsBounded<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        env.add_tyvar_super(self.var.clone(), Rc::unwrap_or_clone(self.sup_ty.clone()));
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
