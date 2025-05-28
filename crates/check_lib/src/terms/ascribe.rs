use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Ascribe, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Ascribe<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let t_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let asc_norm = self.ty.clone().normalize(&mut env.clone());
        let t_kind = t_ty.check_kind(&mut env.clone())?;
        let ty_kind = self.ty.check_kind(env)?;
        t_kind.check_equal(&ty_kind).map_err(to_check_err)?;
        asc_norm.check_equal(&t_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}
