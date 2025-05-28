use crate::{env::CheckEnvironment, to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{SumCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for SumCase<T>
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
        let bound_sum = bound_ty.into_sum().map_err(to_check_err)?;

        let mut left_env = env.clone();
        left_env.add_var(self.left_var.clone(), *bound_sum.left);
        let left_ty = self
            .left_term
            .check(&mut left_env)?
            .normalize(&mut left_env.clone());
        let left_knd = left_ty.check_kind(&mut left_env)?;

        env.add_var(self.right_var.clone(), *bound_sum.right);
        let right_ty = self
            .right_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let right_knd = right_ty.check_kind(env)?;

        left_knd.check_equal(&right_knd).map_err(to_check_err)?;
        left_ty.check_equal(&right_ty).map_err(to_check_err)?;
        Ok(right_ty)
    }
}
