use super::{to_eval_err, Value};
use crate::terms::{Fst, Pair, Snd};
use common::{errors::Error, Eval};

impl Eval<'_> for Pair {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let fst_val = self.fst.eval(_env)?;
        let snd_val = self.snd.eval(_env)?;
        Ok(Value::Pair {
            fst: Box::new(fst_val),
            snd: Box::new(snd_val),
        })
    }
}
impl Eval<'_> for Fst {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let pair_val = self.term.clone().eval(_env)?;
        let (fst, _) = pair_val.into_pair().map_err(to_eval_err)?;
        Ok(fst)
    }
}

impl Eval<'_> for Snd {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let pair_val = self.term.clone().eval(_env)?;
        let (_, snd) = pair_val.into_pair().map_err(to_eval_err)?;
        Ok(snd)
    }
}
