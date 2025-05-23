use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Pair, Term};

impl<T> Eval for Pair<T>
where
    T: Term + Eval,
    PairVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let fst_val = self.fst.eval(env)?;
        let snd_val = self.snd.eval(env)?;
        Ok(PairVal::<T>::new(fst_val, snd_val).into())
    }
}
