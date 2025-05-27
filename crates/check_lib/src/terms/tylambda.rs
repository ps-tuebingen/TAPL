use crate::{env::CheckEnvironment, to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Term, TyLambda},
    types::Forall,
};

impl<T> Typecheck for TyLambda<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Forall<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(env)?;
        self.annot.check_equal(&term_knd).map_err(to_check_err)?;
        Ok(Forall::new(&self.var, self.annot.clone(), term_ty).into())
    }
}
