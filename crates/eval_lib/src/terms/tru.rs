use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Term, True};

impl<T> Eval for True<T>
where
    T: Term + Eval,
    TrueVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TrueVal::new().into())
    }
}
