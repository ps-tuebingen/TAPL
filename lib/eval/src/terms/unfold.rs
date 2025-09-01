use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, Unfold},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Unfold<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Unfold<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let term_fold = term_val.clone().into_fold()?;

        let last_step = EvalStep::unfoldfold(
            Unfold::new(self.ty.clone(), term_val),
            *term_fold.val.clone(),
        );
        let mut steps = term_res.congruence(&move |t| Unfold::new(self.ty.clone(), t).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, *term_fold.val))
    }
}
