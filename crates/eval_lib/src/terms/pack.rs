use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Pack, Term};

impl<T> Eval for Pack<T>
where
    T: Term + Eval,
    PackVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(PackVal::<T>::new(self.inner_ty.clone(), term_val, self.outer_ty.clone()).into())
    }
}
