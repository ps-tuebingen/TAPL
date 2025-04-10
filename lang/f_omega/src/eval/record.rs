use super::Value;
use crate::syntax::terms::Record;
use common::{errors::Error, Eval};
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
        for (label, t) in self.records.into_iter() {
            let val = t.eval(_env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record { records: vals })
    }
}
