use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Let, Term},
};

impl<T> Typecheck for Let<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;
    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?;

        env.add_var(self.var.clone(), bound_ty);
        let in_res = self.in_term.check(env.clone())?;
        let in_ty = in_res.ty().normalize(env.clone());
        in_ty.check_kind(env.clone())?;

        let conc = Conclusion::new(env.clone(), self.clone(), in_ty);
        let deriv = Derivation::lett(conc, bound_res, in_res);
        Ok(deriv)
    }
}
