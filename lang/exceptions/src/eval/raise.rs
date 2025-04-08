use super::{Error, Value};
use crate::syntax::Raise;
use common::Eval;

impl Eval<'_> for Raise {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let exc_val = self.exception.eval(env)?;
        Err(Error::ExceptionVal(exc_val))
    }
}
