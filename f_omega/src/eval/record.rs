use super::{Eval, Value};
use crate::{errors::Error, syntax::terms::Record};
use std::collections::HashMap;

impl Eval for Record {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = HashMap::new();
        for (label, t) in self.records.into_iter() {
            let val = t.eval()?;
            vals.insert(label, val);
        }
        Ok(Value::Record { records: vals })
    }
}
