use crate::Eval;
use errors::eval_error::EvalError;

use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Head, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Head<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Head<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let cons_val = term_val.clone().into_cons()?;

        let last_step =
            EvalStep::head(Head::new(term_val, self.ty.clone()), *cons_val.head.clone());
        let mut steps = term_res.congruence(&move |t| Head::new(t, self.ty.clone()).into());
        steps.push(last_step);

        Ok(EvalTrace::<Lang>::new(steps, *cons_val.head))
    }
}
