use crate::Eval;
use common::errors::FreeVariable;
use syntax::terms::{Term, Variable};
use trace::EvalTrace;

impl<T> Eval for Variable<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::EvalError: From<FreeVariable>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Err(FreeVariable::new(&self.var).into())
    }
}
