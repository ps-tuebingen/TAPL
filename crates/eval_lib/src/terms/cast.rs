use crate::Eval;
use syntax::{
    terms::{Cast, Term},
    types::Type,
};

impl<T, Ty> Eval for Cast<T, Ty>
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
