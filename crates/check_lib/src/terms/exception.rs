use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> Typecheck for Exception<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?;
        Ok(ty_norm)
    }
}
