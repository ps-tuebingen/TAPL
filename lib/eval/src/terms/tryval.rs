use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{App, Term, TryWithVal},
    values::{Raise, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for TryWithVal<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    Lang::Value: Into<Lang::Term>,
    TryWithVal<Lang>: Into<Lang::Term>,
    Raise<Lang>: Into<Lang::Value>,
    App<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (res_steps, res_val) = if let Ok(raise) = term_val.clone().into_raise() {
            let raise_term: Lang::Term = (*raise.val).into();
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
        Ok(EvalTrace::<Lang>::new(steps, res_val))
    }
}
