use super::Value;
use crate::{
    errors::Error,
    terms::{Fst, Pair, Snd},
};
use common::Eval;
impl<'a> Eval<'a> for Pair {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let fst_val = self.fst.eval(_env)?;
        let snd_val = self.snd.eval(_env)?;
        Ok(Value::Pair {
            fst: Box::new(fst_val),
            snd: Box::new(snd_val),
        })
    }
}
impl<'a> Eval<'a> for Fst {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let pair_val = self.term.clone().eval(_env)?;
        let (fst, _) = pair_val
            .into_pair()
            .map_err(|knd| Error::eval(knd, &self))?;
        Ok(fst)
    }
}

impl<'a> Eval<'a> for Snd {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let pair_val = self.term.clone().eval(_env)?;
        let (_, snd) = pair_val
            .into_pair()
            .map_err(|knd| Error::check(knd, &self))?;
        Ok(snd)
    }
}
