use super::{Error, Value};
use crate::syntax::Unit;
use common::Eval;

impl Eval<'_> for Unit {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Error> {
        Ok(self.into())
    }
}
