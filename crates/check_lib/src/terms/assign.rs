use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Assign, Term},
    types::{TypeGroup, Unit as UnitTy},
};

impl<T> Typecheck for Assign<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    UnitTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let lhs_ty = self
            .lhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        lhs_ty.check_kind(&mut env.clone())?.into_star()?;
        let lhs_ref = lhs_ty.into_ref()?;

        let rhs_ty = self
            .rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        rhs_ty.check_kind(env)?.into_star()?;
        lhs_ref.ty.check_equal(&rhs_ty)?;
        Ok(UnitTy::new().into())
    }
}
