use crate::{Eval, errors::EvalError};
use common::errors::FreeVariable;
use syntax::{
    store::Store,
    terms::{Term, Variable},
};
use trace::EvalTrace;

impl<T> Eval for Variable<T>
where
    T: Term + Eval<Term = T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Err(FreeVariable::new(&self.var).into())
    }
}
