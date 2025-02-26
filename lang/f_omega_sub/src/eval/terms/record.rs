use crate::{
    errors::Error,
    eval::{Env, Eval, Value},
    syntax::terms::Record,
};
use std::collections::HashMap;

impl Eval for Record {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let mut vals = HashMap::new();
        for (label, term) in self.records {
            let val = term.eval(env)?;
            vals.insert(label, val);
        }
        Ok(Value::Record { records: vals })
    }
}
