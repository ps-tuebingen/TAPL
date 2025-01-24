use super::{Eval, Value};
use crate::{
    errors::Error,
    syntax::{LambdaSub, TyApp},
    traits::SubstTy,
};

impl Eval for LambdaSub {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::LambdaSub {
            var: self.var,
            sup_ty: self.sup_ty,
            body: *self.body,
        })
    }
}

impl Eval for TyApp {
    fn eval(self) -> Result<Value, Error> {
        let fun_val = self.term.clone().eval()?;
        let (var, _, body) = fun_val.as_lamsub().map_err(|knd| Error::eval(knd, &self))?;
        body.subst_ty(&var, self.ty).eval()
    }
}
