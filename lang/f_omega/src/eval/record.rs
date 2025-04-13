use super::Value;
use crate::syntax::terms::Record;
use common::{errors::Error, Eval};
use std::collections::HashMap;

impl Eval<'_> for Record {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let mut vals = HashMap::new();
        for (label, t) in self.records.into_iter() {
            let val = t.eval(_env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record { records: vals })
    }
}
