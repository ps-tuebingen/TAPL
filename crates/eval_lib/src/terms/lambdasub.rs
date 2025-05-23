use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{LambdaSub, Term};

impl<T> Eval for LambdaSub<T>
where
    T: Term + Eval,
    LambdaSubVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LambdaSubVal::new(&self.var, self.sup_ty, *self.body).into())
    }
}
