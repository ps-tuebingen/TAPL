use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    terms::{Record, RecordProj},
};
use std::collections::HashMap;

impl Eval for Record {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = HashMap::new();
        for (label, term) in self.records {
            let val = term.eval()?;
            vals.insert(label, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval for RecordProj {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.clone().eval()?;
        let recs = val.as_rec().map_err(|knd| Error::eval(knd, &self))?;
        recs.get(&self.label)
            .ok_or(Error::eval(ErrorKind::label(&self.label), &self))
            .cloned()
    }
}
