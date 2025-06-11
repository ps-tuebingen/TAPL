use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    terms::{Fst, Pair, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Fst<T>
where
    T: Term + Eval<Term = T>,
    Fst<T>: Into<T>,
    Pair<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
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
        let pair_val = term_val.clone().into_pair()?;

        let last_step = EvalStep::fst(term_val, *pair_val.snd);
        let mut steps = term_res.congruence(&move |t| Fst::new(t).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            steps,
            *pair_val.fst,
        ))
    }
}
