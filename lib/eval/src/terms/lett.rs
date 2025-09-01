use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    subst::SubstTerm,
    terms::{Let, Term},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Let<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    Let<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let term_subst = self
            .in_term
            .clone()
            .subst(&self.var, &bound_val.clone().into());
        let subst_step = EvalStep::lett(
            Let::new(&self.var, bound_val, *self.in_term.clone()),
            term_subst.clone(),
        );

        let mut steps =
            bound_res.congruence(&move |t| Let::new(&self.var, t, *self.in_term.clone()).into());
        steps.push(subst_step);
        let term_res = term_subst.eval(env)?;
        let val = term_res.val();
        steps.extend(term_res.steps);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
}
