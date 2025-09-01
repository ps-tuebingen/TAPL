use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Pair, Term},
    values::Pair as PairVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Pair<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Pair<Lang>: Into<Lang::Term>,
    PairVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let fst_res = self.fst.clone().eval(env)?;
        let fst_val = fst_res.val();
        let snd_res = self.snd.clone().eval(env)?;
        let snd_val = snd_res.val();

        let fst_steps = fst_res.congruence(&move |t| Pair::new(t, *self.snd.clone()).into());
        let snd_steps = snd_res.congruence(&move |t| Pair::new(*self.fst.clone(), t).into());
        let mut steps = fst_steps;
        steps.extend(snd_steps);
        let val = PairVal::<Lang>::new(fst_val, snd_val);
        Ok(EvalTrace::new(steps, val))
    }
}
