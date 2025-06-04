use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use syntax::{
    env::Environment,
    terms::{Let, Term},
};

impl<T> Typecheck for Let<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;
    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?;

        env.add_var(self.var.clone(), bound_ty);
        let in_ty = self
            .in_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        in_ty.check_kind(env)?;
        Ok(in_ty)
    }
}
