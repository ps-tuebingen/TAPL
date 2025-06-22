use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Nil, Term},
    types::{List, Type},
};

impl<T, Ty> Typecheck for Nil<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: Type + Normalize<Ty> + Kindcheck<Ty>,
    List<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError<Self::Type>> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        ty_norm.check_kind(env.clone())?.into_star()?;

        let conc = Conclusion::new(env.clone(), self.clone(), List::new(ty_norm.clone()).into());
        let deriv = TypingDerivation::nil(conc);
        Ok(deriv.into())
    }
}
