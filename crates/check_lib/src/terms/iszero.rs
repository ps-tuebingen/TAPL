use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{IsZero, Term},
    types::{Bool, TypeGroup},
};

impl<T> Typecheck for IsZero<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError<Self::Type>> {
        let inner_res = self.term.check(env.clone())?;
        let inner_ty = inner_res.ty().normalize(env.clone());
        inner_ty.check_kind(env.clone())?.into_star()?;
        inner_ty.into_nat()?;

        let conc = Conclusion::new(env, self.clone(), Bool::new());
        let deriv = TypingDerivation::iszero(conc, inner_res);
        Ok(deriv.into())
    }
}
