use crate::Eval;
use errors::eval_error::EvalError;
use errors::ValueMismatch;
use syntax::{
    eval_context::EvalContext, language::Language, subst::SubstType, terms::TyApp,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for TyApp<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    TyApp<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let fun_res = self.fun.eval(env)?;
        let fun_val = fun_res.val();
        let (res_steps, res_val) = if let Ok(tylam) = fun_val.clone().into_tylambda() {
            let term_subst = tylam.term.subst_type(&tylam.var, &self.arg);
            let next_step =
                EvalStep::tyappabs(TyApp::new(fun_val, self.arg.clone()), term_subst.clone());
            let term_res = term_subst.eval(env)?;
            let term_val = term_res.val();
            let mut steps = term_res.steps;
            steps.push(next_step);
            (steps, term_val)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            let term_subst = lamsub.term.subst_type(&lamsub.var, &self.arg);
            let next_step =
                EvalStep::tyappabs_sub(TyApp::new(fun_val, self.arg.clone()), term_subst.clone());
            let term_res = term_subst.eval(env)?;
            let term_val = term_res.val();
            let mut steps = term_res.steps;
            steps.push(next_step);
            (steps, term_val)
        } else {
            return Err(
                ValueMismatch::new(fun_val.to_string(), "LambdaSub Term".to_owned()).into(),
            );
        };

        let mut steps = fun_res.congruence(&move |t| TyApp::new(t, self.arg.clone()).into());
        steps.extend(res_steps);
        Ok(EvalTrace::<Lang>::new(steps, res_val))
    }
}
