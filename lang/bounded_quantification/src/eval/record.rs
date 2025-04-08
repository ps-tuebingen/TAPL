use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    syntax::{Projection, Record},
};
use common::Eval;
use std::collections::HashMap;

impl Eval<'_> for Record {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let mut vals = HashMap::new();
        for (lb, t) in self.records.into_iter() {
            let val = t.eval(_env)?;
            vals.insert(lb, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval<'_> for Projection {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let rec_val = self.record.clone().eval(_env)?;
        let records = rec_val.as_record().map_err(|knd| Error::eval(knd, &self))?;
        records.get(&self.label).cloned().ok_or(Error::eval(
            ErrorKind::UndefinedLabel(self.label.clone()),
            &self,
        ))
    }
}
