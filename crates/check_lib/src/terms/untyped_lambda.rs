use crate::Typecheck;
use derivation::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Term, UntypedLambda},
    types::Fun,
    untyped::Untyped,
};

impl<T> Typecheck for UntypedLambda<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
    Untyped: Into<<T as Typecheck>::Type>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        Ok(TypingDerivation::untyped_lambda(Conclusion::new(
            env,
            self.clone(),
            Untyped.into(),
        )))
    }
}
