use crate::{Kindcheck, Normalize, Typecheck, errors::CheckError};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{SomeCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for SomeCase<T>
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
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;

        let option = bound_ty.into_optional()?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), *option.ty);
        let some_res = self.some_term.check(some_env.clone())?;
        let some_ty = some_res.ty().normalize(some_env.clone());
        let some_knd = some_ty.check_kind(some_env)?;

        let none_res = self.none_term.check(env.clone())?;
        let none_ty = none_res.ty().normalize(env.clone());
        let none_knd = none_ty.check_kind(env.clone())?;

        some_knd.check_equal(&none_knd)?;
        some_ty.check_equal(&none_ty)?;

        let conc = Conclusion::new(env, self.clone(), some_ty);
        let deriv = TypingDerivation::somecase(conc, bound_res, some_res, none_res);
        Ok(deriv)
    }
}
