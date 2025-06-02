use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Cast, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Cast<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let term_kind = term_ty.check_kind(&mut env.clone())?;
        let ty_kind = ty_norm.check_kind(env)?;
        term_kind.check_equal(&ty_kind)?;
        Ok(ty_norm)
    }
}
