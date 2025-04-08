use super::Value;
use crate::{
    errors::Error,
    terms::{Fold, Unfold},
};
use common::Eval;

impl Eval<'_> for Fold {
    type Value = Value;
    type Err = Error;
    type Env = ();
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
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.clone().eval(_env)?;
        let (_, val) = val.into_fold().map_err(|knd| Error::eval(knd, &self))?;
        Ok(val)
    }
}
