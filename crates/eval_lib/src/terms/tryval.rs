use crate::{errors::EvalError, Eval};
use syntax::{
    store::Store,
    terms::{App, Term, TryWithVal},
    values::{Raise, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for TryWithVal<T>
where
    T: Term + Eval<Term = T> + From<<T as Eval>::Value>,
    <T as Eval>::Value: Into<T>,
    TryWithVal<T>: Into<T>,
    Raise<<T as Eval>::Value, <<T as Eval>::Value as ValueGroup>::Type>: Into<<T as Eval>::Value>,
    App<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (res_steps, res_val) = if let Ok(raise) = term_val.clone().into_raise() {
            let raise_term: T = (*raise.val).into();
            let next_term = App::new(*self.handler.clone(), raise_term).into();
            let next_step = EvalStep::tryval_catch(
                TryWithVal::new(term_val, *self.handler.clone()),
                next_term.clone(),
            );
            let next_res = next_term.eval(env)?;
            let next_val = next_res.val();
            let mut steps = next_res.steps;
            steps.insert(0, next_step);
            (steps, next_val)
        } else {
            let next_step = EvalStep::tryval_succ(
                TryWithVal::new(term_val.clone(), *self.handler.clone()),
                term_val.clone(),
            );
            (vec![next_step], term_val)
        };

        let mut steps =
            term_res.congruence(&move |t| TryWithVal::new(t, *self.handler.clone()).into());
        steps.extend(res_steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, res_val))
    }
}
