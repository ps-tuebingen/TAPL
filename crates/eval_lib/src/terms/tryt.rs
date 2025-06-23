use crate::{Eval, errors::EvalError};
use syntax::{
    eval_context::EvalContext,
    terms::{Term, Try},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Try<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Try<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (res_steps, res_val) = if term_val.clone().into_exception().is_ok() {
            let next_step = EvalStep::try_catch(
                Try::new(term_val.clone(), *self.handler.clone()),
                *self.handler.clone(),
            );
            let handler_res = self.handler.clone().eval(env)?;
            let handler_val = handler_res.val();
            let mut handler_steps = handler_res.steps;
            handler_steps.insert(0, next_step);
            (handler_steps, handler_val)
        } else {
            let next_step = EvalStep::try_succ(
                Try::new(term_val.clone(), *self.handler.clone()),
                term_val.clone(),
            );
            (vec![next_step], term_val)
        };

        let mut steps = term_res.congruence(&move |t| Try::new(t, *self.handler.clone()).into());
        steps.extend(res_steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, res_val))
    }
}
