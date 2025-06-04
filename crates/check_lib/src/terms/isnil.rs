use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{IsNil, Term},
    types::{Bool, List, TypeGroup},
};

impl<T, Ty> Typecheck for IsNil<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    List<Ty>: Into<Ty>,
    Bool<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<KindMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star()?;
        term_ty.into_list()?;
        Ok(Bool::new().into())
    }
}
