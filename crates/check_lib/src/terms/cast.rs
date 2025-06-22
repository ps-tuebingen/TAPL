use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Cast, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Cast<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = <T as Typecheck>::Term;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        let ty_norm = self.ty.clone().normalize(env.clone());
        let term_kind = term_ty.check_kind(env.clone())?;
        let ty_kind = ty_norm.check_kind(env.clone())?;
        term_kind.check_equal(&ty_kind)?;

        let conc = Conclusion::new(env, self.clone(), ty_norm.clone());
        let deriv = TypingDerivation::cast(conc, term_res);
        Ok(deriv.into())
    }
}
