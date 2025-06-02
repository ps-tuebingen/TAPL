use crate::{errors::ValueMismatch, values::ValueGroup, Eval};
use syntax::{
    subst::SubstTerm,
    terms::{App, Term},
};

impl<T> Eval for App<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Self: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut <T as Eval>::Env) -> Result<<T as Eval>::Value, Self::EvalError> {
        let fun_val = self.fun.eval(env)?;

        let lam = fun_val.into_lambda()?;
        let arg_val: <T as Eval>::Value = self.arg.eval(env)?;
        lam.body.subst(&lam.var, &arg_val.into()).eval(env)
    }
}
