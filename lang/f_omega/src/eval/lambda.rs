use super::Value;
use crate::{errors::Error, syntax::terms::Lambda};
use common::Eval;

impl Eval<'_> for Lambda {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}
