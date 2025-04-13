use super::{Error, Value};
use crate::syntax::Raise;
use common::Eval;

impl Eval<'_> for Raise {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let exc_val = self.exception.eval(env)?;
        Ok(Value::Raise {
            val: Box::new(exc_val),
            cont_ty: self.cont_ty,
            ex_ty: self.ex_ty,
        })
    }
}
