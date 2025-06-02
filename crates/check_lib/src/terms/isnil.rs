use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{IsNil, Term},
    types::{Bool, List, TypeGroup},
};

impl<T, Ty> Typecheck for IsNil<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    List<Ty>: Into<Ty>,
    Bool<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star()?;
        term_ty.into_list()?;
        Ok(Bool::new().into())
    }
}
