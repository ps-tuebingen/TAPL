use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::{FreeTypeVariable, TypeMismatch};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{TypeGroup, TypeVariable},
};
impl<Ty> Subtypecheck<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch> + From<FreeTypeVariable>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: &mut Environment<Ty>) -> Result<(), Self::CheckError> {
        let ty_super = env.get_tyvar_super(&self.v)?;
        let sup_norm = sup.clone().normalize(env);

        if sup_norm.clone().into_top().is_ok() {
            return Ok(());
        }

        if let Ok(v) = sup_norm.clone().into_variable() {
            if v.v == self.v {
                return Ok(());
            }
        }
        ty_super.check_equal(&sup_norm)?;
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
    <Ty as Kindcheck<Ty>>::CheckError: From<FreeTypeVariable>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, env: &mut Environment<Ty>) -> Result<Kind, Self::CheckError> {
        env.get_tyvar_kind(&self.v).map_err(|err| err.into())
    }
}

impl<Ty> Normalize<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, env: &mut Environment<Ty>) -> Ty {
        env.get_tyvar_super(&self.v).unwrap_or(self.into())
    }
}
