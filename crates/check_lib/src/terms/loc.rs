use crate::{CheckEnvironment, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Loc, Term},
    types::Reference,
};

impl<T> Typecheck for Loc<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    Reference<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::Env: CheckEnvironment<CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let loc_ty = env.get_loc(&self.loc)?.normalize(&mut env.clone());
        loc_ty.check_kind(env)?.into_star()?;
        Ok(Reference::new(loc_ty).into())
    }
}
