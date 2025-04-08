use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{Record, RecordProj},
};
use common::Eval;
use std::collections::HashMap;

impl Eval<'_> for Record {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let mut vals = HashMap::new();
        for (lb, term) in self.records {
            let val = term.eval(_env)?;
            vals.insert(lb, val);
        }
        Ok(Value::Record(vals))
    }
}
impl Eval<'_> for RecordProj {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let rec_val = self.record.clone().eval(_env)?;
        let records = rec_val
            .into_record()
            .map_err(|knd| Error::eval(knd, &self))?;
        records
            .get(&self.label)
            .ok_or(Error::eval(
                ErrorKind::UndefinedLabel(self.label.clone()),
                &self,
            ))
            .cloned()
    }
}
