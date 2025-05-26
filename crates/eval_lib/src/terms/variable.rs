use crate::{to_eval_err, Eval};
use common::errors::{Error, ErrorKind};
use syntax::terms::{Term, Variable};

impl<T> Eval for Variable<T>
where
    T: Term + Eval,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Err(to_eval_err(ErrorKind::FreeVariable(self.var)))
    }
}
