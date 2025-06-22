use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use common::errors::UndefinedLabel;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{RecordProj, Term},
    types::TypeGroup,
};

impl<T> Typecheck for RecordProj<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let term_res = self.record.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?;

        let term_rec = match term_ty.clone().into_variable() {
            Ok(v) => env.get_tyvar_super(&v.v)?.normalize(env.clone()),
            Err(_) => term_ty,
        };
        let rec_ty = term_rec.into_record()?;
        let ty = rec_ty
            .records
            .get(&self.label)
            .ok_or(UndefinedLabel::new(&self.label))
            .cloned()?;

        let conc = Conclusion::new(env, self.clone(), ty);
        let deriv = Derivation::recordproj(conc, term_res);
        Ok(deriv)
    }
}
