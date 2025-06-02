use crate::Eval;
use syntax::{
    terms::{Ascribe, Term},
    types::Type,
};

impl<T, Ty> Eval for Ascribe<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        self.term.eval(env)
    }
}
