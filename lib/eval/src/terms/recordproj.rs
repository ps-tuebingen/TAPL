use crate::Eval;
use errors::UndefinedLabel;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{RecordProj, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for RecordProj<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    RecordProj<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.record.eval(env)?;
        let term_val = term_res.val();
        let rec_val = term_val.into_record()?;
        let val = rec_val
            .records
            .get(&self.label)
            .cloned()
            .ok_or(UndefinedLabel::new(&self.label))?;

        let last_step =
            EvalStep::recordproj(RecordProj::new(val.clone(), &self.label), val.clone());

        let mut steps = term_res.congruence(&move |t| RecordProj::new(t, &self.label).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
}
