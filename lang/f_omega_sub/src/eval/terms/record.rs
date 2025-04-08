use super::{Env, Value};
use crate::{errors::Error, syntax::terms::Record};
use common::Eval;

use std::collections::HashMap;

impl<'a> Eval<'a> for Record {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let mut vals = HashMap::new();
        for (label, term) in self.records {
            let val = term.eval(env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record { records: vals })
    }
}
