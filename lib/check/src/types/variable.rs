use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{Top, TypeGroup, TypeVariable},
};
impl<Lang> Subtypecheck for TypeVariable<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        let ty_super = env.get_tyvar_super(&self.v)?;
        let sup_norm = sup.clone().normalize(env.clone());

        if let Ok(top) = sup_norm.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        if let Ok(v) = sup_norm.clone().into_variable()
            && v.v == self.v
        {
            return Ok(SubtypeDerivation::refl(env, self.clone()).into());
        }
        ty_super.check_equal(&sup_norm)?;
        Ok(SubtypeDerivation::refl(env, ty_super).into())
    }
}

impl<Lang> Kindcheck for TypeVariable<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        env.get_tyvar_kind(&self.v).map_err(|err| err.into())
    }
}

impl<Lang> Normalize for TypeVariable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        env.get_tyvar_super(&self.v).unwrap_or(self.into())
    }
}
