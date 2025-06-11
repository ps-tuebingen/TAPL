use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    subst::SubstType,
    terms::{Term, TyApp},
    types::Type,
    values::{Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for TyApp<T, Ty>
where
    T: Term + Eval<Term = T> + SubstType<Ty, Target = T>,
    <T as Eval>::Value: Into<T>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    TyApp<T, Ty>: Into<T>,
    Ty: Type,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
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
            return Err(ValueMismatch::new(fun_val.knd(), ValueKind::LambdaSub).into());
        };

        let mut steps = fun_res.congruence(&move |t| TyApp::new(t, self.arg.clone()).into());
        steps.extend(res_steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, res_val))
    }
}
