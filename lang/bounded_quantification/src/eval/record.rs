use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::{Projection, Record},
};
use std::collections::HashMap;

impl Eval for Record {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = HashMap::new();
        for (lb, t) in self.records.into_iter() {
            let val = t.eval()?;
            vals.insert(lb, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval for Projection {
    fn eval(self) -> Result<Value, Error> {
        let rec_val = self.record.clone().eval()?;
        let records = rec_val.as_record().map_err(|knd| Error::eval(knd, &self))?;
        records.get(&self.label).cloned().ok_or(Error::eval(
            ErrorKind::UndefinedLabel(self.label.clone()),
            &self,
        ))
    }
}
