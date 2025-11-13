use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Top, TypeGroup, TypeVariable},
};
impl<Lang> Subtypecheck for TypeVariable<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let ty_super = env.get_tyvar_super(&self.v)?;

        let sup_norm;
        if features.normalizing {
            let sup_norm_deriv = sup.clone().normalize(env.clone());
            sup_norm = sup_norm_deriv.ret_ty();
            premises.push(sup_norm_deriv);
        } else {
            sup_norm = sup.clone();
        }

        if let Ok(top) = sup_norm.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, premises).into());
        }

        if let Ok(v) = sup_norm.clone().into_variable()
            && v.v == self.v
        {
            return Ok(SubtypeDerivation::refl(env, self.clone(), premises).into());
        }
        ty_super.check_equal(&sup_norm)?;
        Ok(SubtypeDerivation::refl(env, ty_super, premises).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_top(), DerivationRule::sub_var()])
    }
}

impl<Lang> Kindcheck for TypeVariable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let knd = env.get_tyvar_kind(&self.v)?;
        Ok(KindingDerivation::var(&self.v, knd).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_var()])
    }
}

impl<Lang> Normalize for TypeVariable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(env.get_tyvar_super(&self.v).unwrap_or_else(|_| self.into()))
            .into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
