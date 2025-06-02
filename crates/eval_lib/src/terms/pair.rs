use crate::{values::Pair as PairVal, Eval};
use syntax::terms::{Pair, Term};

impl<T> Eval for Pair<T>
where
    T: Term + Eval,
    PairVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let fst_val = self.fst.eval(env)?;
        let snd_val = self.snd.eval(env)?;
        Ok(PairVal::<<T as Eval>::Value>::new(fst_val, snd_val).into())
    }
}
