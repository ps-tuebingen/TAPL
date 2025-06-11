use crate::Eval;
use syntax::{
    terms::{Something, Term},
    values::Something as SomethingVal,
};
use trace::EvalTrace;

impl<T> Eval for Something<T>
where
    T: Term + Eval<Term = T>,
    Something<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    SomethingVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = SomethingVal::<<T as Eval>::Value>::new(term_val);
        let steps = term_res.congruence(&move |t| Something::new(t).into());
        Ok(EvalTrace::new(steps, val))
    }
}
