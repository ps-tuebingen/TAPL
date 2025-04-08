use super::{Env, Value};
use crate::{errors::Error, syntax::terms::Record};
use common::Eval;

use std::collections::HashMap;

impl<'a> Eval<'a> for Record {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let mut vals = HashMap::new();
        for (label, term) in self.records {
            let val = term.eval(env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record { records: vals })
    }
}
