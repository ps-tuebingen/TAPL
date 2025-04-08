use super::{Error, Value};
use crate::syntax::If;
use common::Eval;

impl Eval<'_> for If {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let cond_val = self.ift.eval(env)?;
        match cond_val {
            Value::True => self.thent.eval(env),
            Value::False => self.elset.eval(env),
            v => Err(Error::NotABool(v)),
        }
    }
}
