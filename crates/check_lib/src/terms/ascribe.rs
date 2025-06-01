use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Ascribe, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Ascribe<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let t_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let asc_norm = self.ty.clone().normalize(&mut env.clone());
        let t_kind = t_ty.check_kind(&mut env.clone())?;
        let ty_kind = self.ty.check_kind(env)?;
        t_kind.check_equal(&ty_kind)?;
        asc_norm.check_equal(&t_ty)?;
        Ok(self.ty.clone())
    }
}
