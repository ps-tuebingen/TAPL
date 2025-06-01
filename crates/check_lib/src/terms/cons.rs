use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Cons, Term},
    types::{List, TypeGroup},
};

impl<T, Ty> Typecheck for Cons<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    List<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let hd_ty = self
            .head
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        hd_ty.check_equal(&ty_norm)?;
        hd_ty.check_kind(&mut env.clone())?.into_star()?;

        let tl_ty = self
            .tail
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        tl_ty.check_kind(env)?.into_star()?;
        let list_ty: Self::Type = List::new(ty_norm).into();
        tl_ty.check_equal(&list_ty)?;
        Ok(list_ty)
    }
}
