use crate::Typecheck;
use syntax::{
    env::Environment,
    terms::{Term, Unit},
    types::Unit as UnitTy,
};

impl<T> Typecheck for Unit<T>
where
    T: Term + Typecheck,
    UnitTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        _: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        Ok(UnitTy::new().into())
    }
}
