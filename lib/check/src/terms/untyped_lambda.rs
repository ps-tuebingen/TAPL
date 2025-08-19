use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Term, UntypedLambda},
    types::Fun,
    untyped::Untyped,
};

impl<T> Typecheck for UntypedLambda<T>
where
    T: Term + Typecheck<Term = T>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
    Untyped: Into<<T as Typecheck>::Type>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        Ok(TypingDerivation::untyped_lambda(TypingConclusion::new(
            env,
            self.clone(),
            Untyped.into(),
        ))
        .into())
    }
}
