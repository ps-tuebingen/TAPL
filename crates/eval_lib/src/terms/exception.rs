use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Exception, Term};

impl<T> Eval for Exception<T>
where
    T: Term + Eval,
    ExceptionVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(ExceptionVal::new(self.ty).into())
    }
}
