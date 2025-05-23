use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{False, Term};

impl<T> Eval for False<T>
where
    T: Term + Eval,
    FalseVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(FalseVal::new().into())
    }
}
