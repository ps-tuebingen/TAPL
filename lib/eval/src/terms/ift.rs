use crate::Eval;
use errors::ValueMismatch;
use errors::eval_error::EvalError;
use std::rc::Rc;
use syntax::{eval_context::EvalContext, language::Language, terms::If, values::ValueGroup};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for If<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    If<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let cond_res = self.if_cond.eval(env)?;
        let cond_val = cond_res.val();
        let (next_step, branch_res) = if cond_val.clone().into_true().is_ok() {
            (
                EvalStep::if_true(
                    If::new(
                        cond_val,
                        Rc::unwrap_or_clone(self.then_term.clone()),
                        Rc::unwrap_or_clone(self.else_term.clone()),
                    ),
                    Rc::unwrap_or_clone(self.then_term.clone()),
                ),
                self.then_term.clone().eval(env)?,
            )
        } else if cond_val.clone().into_false().is_ok() {
            (
                EvalStep::if_false(
                    If::new(
                        cond_val,
                        Rc::unwrap_or_clone(self.then_term.clone()),
                        Rc::unwrap_or_clone(self.else_term.clone()),
                    ),
                    Rc::unwrap_or_clone(self.else_term.clone()),
                ),
                self.else_term.clone().eval(env)?,
            )
        } else {
            return Err(
                ValueMismatch::new(cond_val.to_string(), "Boolean Value".to_owned()).into(),
            );
        };
        let branch_val = branch_res.val();

        let mut steps = cond_res.congruence(&move |t| {
            If::new(
                t,
                Rc::unwrap_or_clone(self.then_term.clone()),
                Rc::unwrap_or_clone(self.else_term.clone()),
            )
            .into()
        });
        steps.push(next_step);
        steps.extend(branch_res.steps);

        Ok(EvalTrace::<Lang>::new(steps, branch_val))
    }
}
