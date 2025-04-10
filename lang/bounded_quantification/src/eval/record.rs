use super::{to_eval_err, Value};
use crate::syntax::{Projection, Record};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};
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
        for (lb, t) in self.records.into_iter() {
            let val = t.eval(_env)?;
            vals.insert(lb, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval<'_> for Projection {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let rec_val = self.record.clone().eval(_env)?;
        let records = rec_val.as_record().map_err(to_eval_err)?;
        records
            .get(&self.label)
            .cloned()
            .ok_or(ErrorKind::UndefinedLabel(self.label.clone()))
            .map_err(to_eval_err)
    }
}
