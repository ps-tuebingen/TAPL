use super::{Error, Eval, Value};
use crate::{terms::Fix, traits::subst_term::SubstTerm};

impl Eval for Fix {
    fn eval(self) -> Result<Value, Error> {
        let body_val = self.term.clone().eval()?;
        let (var, _, body) = body_val
            .as_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        let body_subst = body.subst(&var, self.clone().into());
        body_subst.eval()
    }
}
