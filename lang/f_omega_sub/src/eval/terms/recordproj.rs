use super::{Env, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::RecordProj,
};
use common::Eval;

impl<'a> Eval<'a> for RecordProj {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.clone().eval(env)?;
        let rec = val.as_rec().map_err(|knd| Error::eval(knd, self.clone()))?;
        rec.get(&self.label).cloned().ok_or(Error::eval(
            ErrorKind::UndefinedLabel(self.label.clone()),
            self,
        ))
    }
}
