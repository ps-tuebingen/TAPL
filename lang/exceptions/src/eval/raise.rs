use super::{Error, Value};
use crate::syntax::Raise;
use common::Eval;

impl<'a> Eval<'a> for Raise {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let exc_val = self.exception.eval(env)?;
        Err(Error::ExceptionVal(exc_val))
    }
}
