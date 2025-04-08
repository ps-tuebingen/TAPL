use super::Value;
use crate::{
    errors::Error,
    terms::{Fst, Pair, Snd},
};
use common::Eval;
impl Eval<'_> for Pair {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
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
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let pair_val = self.term.clone().eval(_env)?;
        let (fst, _) = pair_val
            .into_pair()
            .map_err(|knd| Error::eval(knd, &self))?;
        Ok(fst)
    }
}

impl Eval<'_> for Snd {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let pair_val = self.term.clone().eval(_env)?;
        let (_, snd) = pair_val
            .into_pair()
            .map_err(|knd| Error::check(knd, &self))?;
        Ok(snd)
    }
}
