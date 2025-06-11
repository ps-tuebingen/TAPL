use crate::Eval;
use syntax::{
    terms::{Cons, Term},
    types::Type,
    values::{Cons as ConsVal, ValueGroup},
};
use trace::EvalTrace;

impl<T, V, Ty> Eval for Cons<T, Ty>
where
    T: Term + Eval<Term = T, Value = V>,
    Ty: Type,
    V: ValueGroup + Into<T>,
    Cons<T, Ty>: Into<T>,
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
        let hd_res = self.head.clone().eval(env)?;
        let hd_val = hd_res.val();

        let tail_res = self.tail.clone().eval(env)?;
        let tail_val = tail_res.val();

        let val = ConsVal::<V, Ty>::new(hd_val, tail_val, self.ty.clone()).into();

        let ty_ = self.ty.clone();
        let mut steps =
            hd_res.congruence(&move |t| Cons::new(t, *self.tail.clone(), ty_.clone()).into());

        steps.extend(
            tail_res.congruence(&move |t| Cons::new(*self.head.clone(), t, self.ty.clone()).into()),
        );
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
