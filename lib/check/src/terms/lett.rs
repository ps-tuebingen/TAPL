use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Let, Term},
};

impl<T> Typecheck for Let<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?;

        env.add_var(self.var.clone(), bound_ty);
        let in_res = self.in_term.check(env.clone())?;
        let in_ty = in_res.ty().normalize(env.clone());
        in_ty.check_kind(env.clone())?;

        let conc = Conclusion::new(env.clone(), self.clone(), in_ty);
        let deriv = TypingDerivation::lett(conc, bound_res, in_res);
        Ok(deriv)
    }
}
