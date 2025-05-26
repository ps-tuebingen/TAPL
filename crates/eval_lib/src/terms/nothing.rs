use crate::{values::Nothing as NothingVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Nothing, Term},
    types::Type,
};

impl<T, Ty> Eval for Nothing<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    NothingVal<T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NothingVal::<T>::new(self.ty).into())
    }
}
