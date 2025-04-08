use super::Value;
use crate::{errors::Error, syntax::terms::TyLambda};
use common::Eval;

impl Eval<'_> for TyLambda {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::TyLambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}
