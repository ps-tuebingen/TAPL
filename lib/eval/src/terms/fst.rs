use crate::Eval;
use errors::eval_error::EvalError;

use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Fst, Pair, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Fst<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Fst<Lang>: Into<Lang::Term>,
    Pair<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let pair_val = term_val.clone().into_pair()?;

        let last_step = EvalStep::fst(term_val, *pair_val.snd);
        let mut steps = term_res.congruence(&move |t| Fst::new(t).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, *pair_val.fst))
    }
}
