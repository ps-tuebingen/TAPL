use crate::{env::EvalEnvironment, errors::ValueMismatch, values::ValueGroup, Eval};
use syntax::terms::{Deref, Term};

impl<T> Eval for Deref<T>
where
    T: Term + Eval,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.clone().eval(env)?;
        let loc_val = term_val.into_loc()?;
        env.get_location(loc_val.loc)
    }
}
