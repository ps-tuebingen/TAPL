use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Something, Term};

impl<T> Eval for Something<T>
where
    T: Term + Eval,
    SomethingVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(SomethingVal::<T>::new(term_val).into())
    }
}
