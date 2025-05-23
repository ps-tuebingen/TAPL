use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Nil, Term};

impl<T> Eval for Nil<T>
where
    T: Term + Eval,
    NilVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NilVal::<T>::new(self.ty).into())
    }
}
