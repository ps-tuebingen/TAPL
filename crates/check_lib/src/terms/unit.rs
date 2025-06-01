use crate::Typecheck;
use syntax::{
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
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        Ok(UnitTy::new().into())
    }
}
