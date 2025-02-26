use crate::{
    errors::Error,
    eval::{Env, Eval, Value},
    syntax::terms::Pack,
};

impl Eval for Pack {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let val = self.term.eval(env)?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(val),
            outer_ty: self.outer_ty,
        })
    }
}
