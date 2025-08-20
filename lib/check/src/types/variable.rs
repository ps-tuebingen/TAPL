use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Top, TypeGroup, TypeVariable},
};
impl<Ty> Subtypecheck for TypeVariable<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty> + Normalize<Ty>,
    Top<Ty>: Into<Ty>,
    TypeVariable<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
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

impl<Ty> Kindcheck<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
{
    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, CheckError> {
        env.get_tyvar_kind(&self.v).map_err(|err| err.into())
    }
}

impl<Ty> Normalize<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, env: Environment<Ty>) -> Ty {
        env.get_tyvar_super(&self.v).unwrap_or(self.into())
    }
}
