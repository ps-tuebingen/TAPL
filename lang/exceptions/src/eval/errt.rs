use super::{Error, Value};
use crate::syntax::Error as ErrT;
use common::Eval;

impl Eval for ErrT {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, _: &mut Self::Env) -> Result<Value, Error> {
        Err(Error::Exception)
    }
}
