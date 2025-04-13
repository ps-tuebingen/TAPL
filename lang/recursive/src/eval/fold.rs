use super::{to_eval_err, Value};
use crate::terms::{Fold, Unfold};
use common::{errors::Error, Eval};

impl Eval<'_> for Fold {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(_env)?;
        Ok(Value::Fold {
            ty: self.ty,
            val: Box::new(val),
        })
    }
}

impl Eval<'_> for Unfold {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.clone().eval(_env)?;
        let (_, val) = val.into_fold().map_err(to_eval_err)?;
        Ok(val)
    }
}
