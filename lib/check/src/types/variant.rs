use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::{check_error::CheckError, UndefinedLabel};
use syntax::{
    env::Environment,
    language::Language,
    types::{Top, TypeGroup, Variant},
};

impl<Lang> Subtypecheck for Variant<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Variant<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        let sup_var = sup.clone().into_variant()?;
        let mut inner_res = vec![];
        for (lb, ty) in sup_var.variants.iter() {
            let self_ty = self.variants.get(lb).ok_or(UndefinedLabel::new(lb))?;
            inner_res.push(self_ty.check_subtype(ty, env.clone())?);
        }
        Ok(SubtypeDerivation::variant(env, self.clone(), sup.clone(), inner_res).into())
    }
}
