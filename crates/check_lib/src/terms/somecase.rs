use crate::{env::CheckEnvironment, to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{SomeCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for SomeCase<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;

        let option = bound_ty.into_optional().map_err(to_check_err)?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), *option.ty);
        let some_ty = self
            .some_term
            .check(&mut some_env.clone())?
            .normalize(&mut some_env.clone());
        let some_knd = some_ty.check_kind(&mut some_env)?;

        let none_ty = self
            .none_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let none_knd = none_ty.check_kind(env)?;

        some_knd.check_equal(&none_knd).map_err(to_check_err)?;
        some_ty.check_equal(&none_ty).map_err(to_check_err)?;
        Ok(some_ty)
    }
}
