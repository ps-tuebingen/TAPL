use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{ListCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for ListCase<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?.into_star()?;
        let bound_list = bound_ty.clone().into_list()?;

        let nil_ty = self
            .nil_rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let nil_kind = nil_ty.check_kind(&mut env.clone())?;

        env.add_var(self.cons_fst.clone(), *bound_list.ty);
        env.add_var(self.cons_rst.clone(), bound_ty);
        let cons_ty = self
            .cons_rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let cons_kind = cons_ty.check_kind(env)?;

        nil_kind.check_equal(&cons_kind)?;
        nil_ty.check_equal(&cons_ty)?;
        Ok(cons_ty)
    }
}
