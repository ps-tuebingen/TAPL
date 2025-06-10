use crate::Eval;
use syntax::{
    terms::{Cons, Term},
    types::Type,
    values::{Cons as ConsVal, ValueGroup},
};
use trace::EvalTrace;

impl<T, V, Ty> Eval for Cons<T, Ty>
where
    T: Term + Eval<Value = V>,
    Ty: Type,
    V: ValueGroup,
    ConsVal<V, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let hd_val = self.head.eval(env)?;
        let tail_val = self.tail.eval(env)?;
        Ok(ConsVal::<V, Ty>::new(hd_val, tail_val, self.ty).into())
    }
}
