use super::{Eval, Value};
use crate::{errors::Error, syntax::terms::TyApp, traits::SubstTy};

impl Eval for TyApp {
    fn eval(self) -> Result<Value, Error> {
        let fun_val = self.fun.clone().eval()?;
        let (var, _, body) = fun_val
            .as_tylambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        body.subst_ty(&var, self.arg).eval()
    }
}
