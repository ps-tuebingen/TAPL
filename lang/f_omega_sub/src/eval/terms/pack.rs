use super::{Env, Value};
use crate::syntax::terms::Pack;
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for Pack {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(env)?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(val),
            outer_ty: self.outer_ty,
        })
    }
}
