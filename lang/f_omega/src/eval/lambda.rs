use super::Value;
use crate::syntax::terms::Lambda;
use common::{errors::Error, Eval};

impl Eval<'_> for Lambda {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}
