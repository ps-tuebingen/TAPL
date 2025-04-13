use super::{Error, Value};
use crate::syntax::Try;
use common::Eval;

impl Eval<'_> for Try {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env);
        if term_val.is_err() {
            self.handler.eval(env)
        } else {
            term_val
        }
    }
}
