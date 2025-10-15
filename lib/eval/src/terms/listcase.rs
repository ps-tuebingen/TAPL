use crate::Eval;
use errors::eval_error::EvalError;
use errors::ValueMismatch;
use syntax::{
    eval_context::EvalContext, language::Language, subst::SubstTerm, terms::ListCase,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang> + From<Lang::Value>,
    ListCase<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, val) = if bound_val.clone().into_nil().is_ok() {
            let next_step = EvalStep::listcase_nil(
                ListCase::new(
                    bound_val,
                    *self.nil_rhs.clone(),
                    &self.cons_fst,
                    &self.cons_rst,
                    *self.cons_rhs.clone(),
                ),
                *self.nil_rhs.clone(),
            );
            let nil_res = self.nil_rhs.clone().eval(env)?;
            let nil_val = nil_res.val();
            let mut steps = nil_res.steps;
            steps.insert(0, next_step);
            (steps, nil_val)
        } else if let Ok(cons) = bound_val.clone().into_cons() {
            let cons_subst = self
                .cons_rhs
                .clone()
                .subst(&self.cons_fst, &((*cons.head).into()))
                .subst(&self.cons_rst, &((*cons.tail).into()));
            let next_step = EvalStep::listcase_cons(
                ListCase::new(
                    bound_val,
                    *self.nil_rhs.clone(),
                    &self.cons_fst,
                    &self.cons_rst,
                    *self.cons_rhs.clone(),
                ),
                cons_subst.clone(),
            );
            let cons_res = cons_subst.eval(env)?;
            let cons_val = cons_res.val();
            let mut steps = cons_res.steps;
            steps.insert(0, next_step);
            (steps, cons_val)
        } else {
            return Err(ValueMismatch::new(bound_val.to_string(), "List".to_owned()).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            ListCase::new(
                t,
                *self.nil_rhs.clone(),
                &self.cons_fst,
                &self.cons_rst,
                *self.cons_rhs.clone(),
            )
            .into()
        });
        steps.extend(res_steps);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
}
