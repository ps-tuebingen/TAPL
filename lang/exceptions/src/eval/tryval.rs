use super::{Error, Value};
use crate::syntax::{App, TryWithVal};
use common::Eval;

impl Eval<'_> for TryWithVal {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
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
