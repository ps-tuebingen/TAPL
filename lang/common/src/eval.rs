use crate::errors::Error;

pub trait Eval<'a> {
    type Value;
    type Env;
    fn eval_start(self) -> Result<Self::Value, Error>;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Error>;
}
