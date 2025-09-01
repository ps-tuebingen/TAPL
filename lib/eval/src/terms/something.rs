use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Something, Term},
    values::Something as SomethingVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Something<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Something<Lang>: Into<Lang::Term>,
    SomethingVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = SomethingVal::<Lang>::new(term_val);
        let steps = term_res.congruence(&move |t| Something::new(t).into());
        Ok(EvalTrace::new(steps, val))
    }
}
