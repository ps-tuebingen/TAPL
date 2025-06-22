use crate::{errors::CheckError, Typecheck};
use derivation::{Conclusion, Derivation};
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
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        Ok(Derivation::untyped_lambda(Conclusion::new(
            env,
            self.clone(),
            Untyped.into(),
        )))
    }
}
