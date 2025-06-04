use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{Left, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Left<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    <Ty as Kindcheck<Ty>>::CheckError: From<KindMismatch>,
    <T as Typecheck>::CheckError:
        From<KindMismatch> + From<TypeMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let left_ty = self
            .left_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let left_knd = left_ty.check_kind(&mut env.clone())?;
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let sum_ty = ty_norm.into_sum()?;
        let sum_kind = sum_ty.check_kind(env)?;
        left_knd.check_equal(&sum_kind)?;
        sum_ty.left.check_equal(&left_ty)?;
        Ok(self.ty.clone())
    }
}
