use crate::{values::Tuple as TupleVal, Eval};
use common::errors::Error;
use syntax::terms::{Term, Tuple};

impl<T> Eval for Tuple<T>
where
    T: Term + Eval,
    TupleVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut vals = vec![];
        for t in self.terms.into_iter() {
            let val = t.eval(env)?;
            vals.push(val);
        }
        Ok(TupleVal::<<T as Eval>::Value>::new(vals).into())
    }
}
