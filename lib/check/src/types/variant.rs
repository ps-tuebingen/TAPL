use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::{TypeMismatch, UndefinedLabel, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Top, TypeGroup, Variant},
};

impl<Lang> Subtypecheck for Variant<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Some(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(
                env,
                self.clone(),
                top.kind,
                self.span,
                Vec::new(),
            )
            .into());
        }

        let sup_var = sup.clone().into_variant().ok_or_else(|| {
            TypeMismatch::new(sup.to_string(), "Variant Type".to_string(), self.span)
        })?;
        let mut inner_res = vec![];
        for (lb, ty) in &sup_var.variants {
            let self_ty = self
                .variants
                .get(lb)
                .ok_or_else(|| UndefinedLabel::new(lb, self.span))?;
            inner_res.push(self_ty.check_subtype(ty, env.clone())?);
        }
        Ok(SubtypeDerivation::variant(env, self.clone(), sup.clone(), inner_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_top(), DerivationRule::sub_variant()])
    }
}
