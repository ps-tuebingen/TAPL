use super::{Error, Value};
use crate::syntax::Lambda;
use common::Eval;

impl<'a> Eval<'a> for Lambda {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: Self::Env) -> Result<Value, Error> {
        Ok(self.into())
    }
}
