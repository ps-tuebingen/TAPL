use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{Record, RecordProj},
};
use common::Eval;
use std::collections::HashMap;

impl Eval<'_> for Record {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        let mut vals = HashMap::new();
        for (label, term) in self.records {
            let val = term.eval(_env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval<'_> for RecordProj {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        let val = self.term.clone().eval(_env)?;
        let recs = val.as_rec().map_err(|knd| Error::eval(knd, &self))?;
        recs.get(&self.label)
            .ok_or(Error::eval(ErrorKind::label(&self.label), &self))
            .cloned()
    }
}
