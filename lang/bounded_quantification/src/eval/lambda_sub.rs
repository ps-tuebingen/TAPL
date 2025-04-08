use super::Value;
use crate::{
    errors::Error,
    syntax::{LambdaSub, TyApp},
    traits::SubstTy,
};
use common::Eval;

impl Eval<'_> for LambdaSub {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::LambdaSub {
            var: self.var,
            sup_ty: self.sup_ty,
            body: *self.body,
        })
    }
}

impl Eval<'_> for TyApp {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let fun_val = self.term.clone().eval(_env)?;
        let (var, _, body) = fun_val.as_lamsub().map_err(|knd| Error::eval(knd, &self))?;
        body.subst_ty(&var, self.ty).eval(_env)
    }
}
