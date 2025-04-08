use super::{Error, Value};
use crate::syntax::Lambda;
use common::Eval;

impl Eval<'_> for Lambda {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(self.into())
    }
}
