use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{False, Term},
    types::{Bool, Type},
};

impl<T> Typecheck for False<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type:
        Type + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Bool::new().into())
    }
}
