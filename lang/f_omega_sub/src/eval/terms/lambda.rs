use super::{Env, Eval, Value};
use crate::{errors::Error, syntax::terms::Lambda};

impl Eval for Lambda {
    type Target = Value;
    fn eval(self, _: &mut Env) -> Result<Self::Target, Error> {
        Ok(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}
