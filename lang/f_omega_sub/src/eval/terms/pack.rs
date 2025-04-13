use super::{Env, Value};
use crate::syntax::terms::Pack;
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for Pack {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(val),
            outer_ty: self.outer_ty,
        })
    }
}
