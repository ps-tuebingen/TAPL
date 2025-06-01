use crate::{CheckEnvironment, Kindcheck, Normalize, Typecheck};
use syntax::terms::{Let, Term};

impl<T> Typecheck for Let<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
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
