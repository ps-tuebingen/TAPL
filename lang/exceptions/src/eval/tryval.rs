use super::{to_eval_err, Value};
use crate::syntax::{App, TryWithVal};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for TryWithVal {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let term_evaled = self.term.eval(env)?;
        if let Value::Raise {
            val,
            cont_ty: _,
            ex_ty: _,
        } = term_evaled
        {
            App {
                fun: self.handler,
                arg: Box::new((*val).into()),
            }
            .eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: term_evaled.to_string(),
                expected: "Raised Exception".to_owned(),
            }))
        }
    }
}
