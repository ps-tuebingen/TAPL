use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{FreeTypeVariable, TypeMismatch, UndefinedLabel};
use syntax::{
    env::Environment,
    terms::{RecordProj, Term},
    types::TypeGroup,
};

impl<T> Typecheck for RecordProj<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError:
        From<UndefinedLabel> + From<TypeMismatch> + From<FreeTypeVariable>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let term_ty = self
            .record
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(&mut env.clone())?;

        let term_rec = match term_ty.clone().into_variable() {
            Ok(v) => env.get_tyvar_super(&v.v)?.normalize(env),
            Err(_) => term_ty,
        };
        let rec_ty = term_rec.into_record()?;
        rec_ty
            .records
            .get(&self.label)
            .ok_or(UndefinedLabel::new(&self.label).into())
            .cloned()
    }
}
