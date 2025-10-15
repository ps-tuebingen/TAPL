use crate::Eval;
use errors::ValueMismatch;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{False as FalseT, IsNil, Term, True as TrueT},
    values::{False, True, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for IsNil<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    IsNil<Lang>: Into<Lang::Term>,
    True<Lang>: Into<Lang::Value>,
    TrueT<Lang>: Into<Lang::Term>,
    False<Lang>: Into<Lang::Value>,
    FalseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (step, val) = if term_val.clone().into_nil().is_ok() {
            let last_step = EvalStep::isnil_true(self.ty.clone());
            (last_step, True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            let last_step = EvalStep::isnil_false(self.ty.clone());
            (last_step, False::new().into())
        } else {
            return Err(ValueMismatch::new(term_val.to_string(), "List".to_owned()).into());
        };
        let mut steps = term_res.congruence(&move |t| IsNil::new(t, self.ty.clone()).into());
        steps.push(step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
}
