use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::UndefinedLabel;
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{RecordProj, Term},
    types::TypeGroup,
};

impl<T> Typecheck for RecordProj<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
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
        let deriv = TypingDerivation::recordproj(conc, term_res);
        Ok(deriv)
    }
}
