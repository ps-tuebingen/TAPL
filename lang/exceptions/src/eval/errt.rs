use super::{Error, Value};
use crate::syntax::Error as ErrT;
use common::Eval;

impl Eval<'_> for ErrT {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::Exception(self.ty))
    }
}
