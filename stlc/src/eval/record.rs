use super::{Eval, Value};
use crate::terms::syntax::{Record, RecordProj};
use std::collections::HashMap;

impl Eval for Record {
    fn eval(self) -> Option<Value> {
        let mut vals = HashMap::new();
        for (label, term) in self.records.into_iter() {
            let val = term.eval()?;
            vals.insert(label, val);
        }
        Some(Value::Record(vals))
    }
}

impl Eval for RecordProj {
    fn eval(self) -> Option<Value> {
        match self.record.eval() {
            Some(Value::Record(records)) => records.get(&self.label).cloned(),
            _ => None,
        }
    }
}
