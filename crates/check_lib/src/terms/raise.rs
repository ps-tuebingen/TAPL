use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Raise, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Raise<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ex_norm = self.exception_ty.clone().normalize(&mut env.clone());
        let cont_norm = self.cont_ty.clone().normalize(&mut env.clone());

        let ex_knd = ex_norm.check_kind(&mut env.clone())?;
        self.cont_ty.check_kind(&mut env.clone())?;

        let err_ty = self
            .exception
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let err_knd = err_ty.check_kind(env)?;

        ex_knd.check_equal(&err_knd).map_err(to_check_err)?;
        ex_norm.check_equal(&err_ty).map_err(to_check_err)?;

        Ok(cont_norm.clone())
    }
}
