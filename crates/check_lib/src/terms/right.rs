use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Right, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Right<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let right_ty = self
            .right_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let right_knd = right_ty.check_kind(&mut env.clone())?;

        let sum_ty = self
            .ty
            .clone()
            .normalize(&mut env.clone())
            .into_sum()
            .map_err(to_check_err)?;
        let sum_knd = sum_ty.check_kind(env)?;

        right_knd.check_equal(&sum_knd).map_err(to_check_err)?;
        sum_ty.right.check_equal(&right_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}
