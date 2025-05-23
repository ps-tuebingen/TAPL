use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Cons, Term};

impl<T> Eval for Cons<T>
where
    T: Term + Eval,
    ConsVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let hd_val = self.head.eval(env)?;
        let tail_val = self.tail.eval(env)?;
        Ok(ConsVal::<T>::new(hd_val, tail_val, self.ty).into())
    }
}
