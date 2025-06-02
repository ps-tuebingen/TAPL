use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Nothing, Term},
    types::{Optional, Type},
};

impl<T, Ty> Typecheck for Nothing<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    Optional<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?.into_star()?;
        Ok(Optional::new(ty_norm.clone()).into())
    }
}
