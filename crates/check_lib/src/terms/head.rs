use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Head, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Head<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let list_ty = term_ty.into_list().map_err(to_check_err)?;
        Ok(*list_ty.ty)
    }
}
