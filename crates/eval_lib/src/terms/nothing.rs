use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Nothing, Term};

impl<T> Eval for Nothing<T>
where
    T: Term + Eval,
    NothingVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NothingVal::<T>::new(self.ty).into())
    }
}
