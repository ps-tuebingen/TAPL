use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Left, Term},
    values::Left as LeftVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Left<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Left<Lang>: Into<Lang::Term>,
    LeftVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let left_res = self.left_term.eval(env)?;
        let left_val = left_res.val();
        let val = LeftVal::<Lang>::new(left_val, self.ty.clone());
        let steps = left_res.congruence(&move |t| Left::new(t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }
}
