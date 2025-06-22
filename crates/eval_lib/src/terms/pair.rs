use crate::{Eval, errors::EvalError};
use syntax::{
    store::Store,
    terms::{Pair, Term},
    values::Pair as PairVal,
};
use trace::EvalTrace;

impl<T> Eval for Pair<T>
where
    T: Term + Eval<Term = T>,
    Pair<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    PairVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let fst_res = self.fst.clone().eval(env)?;
        let fst_val = fst_res.val();
        let snd_res = self.snd.clone().eval(env)?;
        let snd_val = snd_res.val();

        let fst_steps = fst_res.congruence(&move |t| Pair::new(t, *self.snd.clone()).into());
        let snd_steps = snd_res.congruence(&move |t| Pair::new(*self.fst.clone(), t).into());
        let mut steps = fst_steps;
        steps.extend(snd_steps);
        let val = PairVal::<<T as Eval>::Value>::new(fst_val, snd_val);
        Ok(EvalTrace::new(steps, val))
    }
}
