use super::{to_eval_err, Value};
use crate::{
    syntax::{LambdaSub, TyApp},
    traits::SubstTy,
};
use common::{errors::Error, Eval};

impl Eval<'_> for LambdaSub {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::LambdaSub {
            var: self.var,
            sup_ty: self.sup_ty,
            body: *self.body,
        })
    }
}

impl Eval<'_> for TyApp {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let fun_val = self.term.clone().eval(_env)?;
        let (var, _, body) = fun_val.as_lamsub().map_err(to_eval_err)?;
        body.subst_ty(&var, self.ty).eval(_env)
    }
}
