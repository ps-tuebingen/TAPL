use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Cons, Term},
    values::Cons as ConsVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Cons<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Cons<Lang>: Into<Lang::Term>,
    ConsVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let hd_res = self.head.clone().eval(env)?;
        let hd_val = hd_res.val();

        let tail_res = self.tail.clone().eval(env)?;
        let tail_val = tail_res.val();

        let val = ConsVal::<Lang>::new(hd_val, tail_val, self.ty.clone()).into();

        let ty_ = self.ty.clone();
        let mut steps =
            hd_res.congruence(&move |t| Cons::new(t, *self.tail.clone(), ty_.clone()).into());

        steps.extend(
            tail_res.congruence(&move |t| Cons::new(*self.head.clone(), t, self.ty.clone()).into()),
        );
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
}
