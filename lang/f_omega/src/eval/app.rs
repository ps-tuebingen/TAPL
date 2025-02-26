use super::{Eval, Value};
use crate::{errors::Error, syntax::terms::App, traits::SubstTerm};

impl Eval for App {
    fn eval(self) -> Result<Value, Error> {
        let fun_val = self.fun.clone().eval()?;
        let (var, _, body) = fun_val.as_lambda().map_err(|knd| Error::eval(knd, &self))?;
        body.subst(&var, *self.arg).eval()
    }
}
