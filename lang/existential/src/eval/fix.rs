use super::{Error, Value};
use crate::{terms::Fix, traits::subst_term::SubstTerm};
use common::Eval;

impl Eval<'_> for Fix {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        let body_val = self.term.clone().eval(_env)?;
        let (var, _, body) = body_val
            .as_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        let body_subst = body.subst(&var, self.clone().into());
        body_subst.eval(_env)
    }
}
