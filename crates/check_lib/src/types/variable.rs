use crate::{env::CheckEnvironment, Kindcheck, Normalize, Subtypecheck};
use common::errors::TypeMismatch;
use syntax::{
    kinds::Kind,
    types::{TypeGroup, TypeVariable},
};
impl<Ty> Subtypecheck<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty, Env = <Ty as Subtypecheck<Ty>>::Env>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
    <Ty as Subtypecheck<Ty>>::Env:
        CheckEnvironment<CheckError = <Ty as Subtypecheck<Ty>>::CheckError>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Self::CheckError> {
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
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Self::CheckError> {
        env.get_tyvar_kind(&self.v)
    }
}

impl<Ty> Normalize<Ty> for TypeVariable<Ty>
where
    Ty: TypeGroup + Normalize<Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.get_tyvar_super(&self.v).unwrap_or(self.into())
    }
}
