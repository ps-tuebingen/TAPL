use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Term, Tuple};

impl<T> Eval for Tuple<T>
where
    T: Term + Eval,
    TupleVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut vals = vec![];
        for t in self.terms.into_iter() {
            let val = t.eval(env)?;
            vals.push(val);
        }
        Ok(TupleVal::<T>::new(vals).into())
    }
}
