use super::{Error, Value};
use crate::syntax::Lambda;
use common::Eval;

impl Eval<'_> for Lambda {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Error> {
        Ok(self.into())
    }
}
