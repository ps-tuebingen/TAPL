use super::{errors::Error, Eval, Value};
use crate::syntax::{Record, RecordProj};
use std::collections::HashMap;

impl Eval for Record {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = HashMap::new();
        for (label, term) in self.records.into_iter() {
            let val = term.eval()?;
            vals.insert(label, val);
        }
        Ok(Value::Record(vals))
    }
}

impl Eval for RecordProj {
    fn eval(self) -> Result<Value, Error> {
        match self.record.eval()? {
            Value::Record(records) => {
                records
                    .get(&self.label)
                    .cloned()
                    .ok_or(Error::UndefinedLabel {
                        label: self.label.clone(),
                    })
            }
            val => Err(Error::BadValue { val }),
        }
    }
}
