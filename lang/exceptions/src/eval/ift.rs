use super::{to_eval_err, Value};
use crate::syntax::If;
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for If {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let cond_val = self.ift.eval(env)?;
        match cond_val {
            Value::True => self.thent.eval(env),
            Value::False => self.elset.eval(env),
            v => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: v.to_string(),
                expected: "Boolean".to_owned(),
            })),
        }
    }
}
