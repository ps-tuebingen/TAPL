use crate::Eval;
use common::errors::FreeVariable;
use syntax::terms::{Term, Variable};

impl<T> Eval for Variable<T>
where
    T: Term + Eval,
    <T as Eval>::EvalError: From<FreeVariable>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Err(FreeVariable::new(&self.var).into())
    }
}
