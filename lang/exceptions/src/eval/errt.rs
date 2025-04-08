use super::{Error, Value};
use crate::syntax::Error as ErrT;
use common::Eval;

impl Eval<'_> for ErrT {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval(self, _: Self::Env) -> Result<Self::Value, Self::Err> {
        Err(Error::Exception)
    }
}
