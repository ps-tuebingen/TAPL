use crate::{CheckEnvironment, Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Term, TyLambda},
    types::Forall,
};

impl<T> Typecheck for TyLambda<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    Forall<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(env)?;
        self.annot.check_equal(&term_knd)?;
        Ok(Forall::new(&self.var, self.annot.clone(), term_ty).into())
    }
}
