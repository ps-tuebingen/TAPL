use crate::Eval;
use syntax::{
    terms::{Cast, Term},
    types::Type,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Cast<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        self.term.eval(env)
    }
}
