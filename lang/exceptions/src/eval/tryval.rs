use super::{Error, Value};
use crate::syntax::{App, TryWithVal};
use common::Eval;

impl Eval<'_> for TryWithVal {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let term_evaled = self.term.eval(env);
        if let Err(Error::ExceptionVal(val)) = term_evaled {
            App {
                fun: self.handler,
                arg: Box::new(val.into()),
            }
            .eval(env)
        } else {
            term_evaled
        }
    }
}
