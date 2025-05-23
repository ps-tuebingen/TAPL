use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Right, Term};

impl<T> Eval for Right<T>
where
    T: Term + Eval,
    RightVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let right_val = self.right_term.eval(env)?;
        Ok(RightVal::<T>::new(right_val, self.ty.clone()).into())
    }
}
