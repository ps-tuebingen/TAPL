use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::terms::{App, Term};

impl<T> Eval for App<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut <T as Eval>::Env) -> Result<<T as Eval>::Value, Error> {
        let fun_val = self.fun.eval(env)?;

        let lam = fun_val.into_lambda().map_err(to_eval_err)?;
        let arg_val: <T as Eval>::Value = self.arg.eval(env)?;
        lam.body.subst(&lam.var, &arg_val.into()).eval(env)
    }
}
