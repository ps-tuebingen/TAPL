use super::{Error, Value};
use crate::syntax::Raise;
use common::Eval;

impl Eval for Raise {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, env: &mut Self::Env) -> Result<Value, Error> {
        let exc_val = self.exception.eval(env)?;
        Err(Error::ExceptionVal(exc_val))
    }
}
