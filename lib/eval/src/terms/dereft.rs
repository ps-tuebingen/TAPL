use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Deref, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Deref<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Deref<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.clone().eval(env)?;
        let term_val = term_res.val();
        let loc_val = term_val.clone().into_loc()?;

        let loc_val = env.get_location(loc_val.loc)?;
        let last_step = EvalStep::deref(term_val, loc_val.clone());
        let mut steps = term_res.congruence(&move |t| Deref::new(t).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, loc_val))
    }
}
