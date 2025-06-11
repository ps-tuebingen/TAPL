use crate::Eval;
use common::errors::ValueMismatch;
use syntax::values::ValueGroup;
use syntax::{
    subst::SubstTerm,
    terms::{App, Term},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for App<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Self: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut <T as Eval>::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let fun_res = self.fun.clone().eval(env)?;
        let fun_val = fun_res.val();
        let lam = fun_val.into_lambda()?;
        let mut steps = fun_res.congruence(&move |t| App::new(t, *self.arg.clone()).into());

        let arg_res = self.arg.clone().eval(env)?;
        let arg_val: <T as Eval>::Value = arg_res.val();
        steps.extend(arg_res.congruence(&move |t| App::new(*self.fun.clone(), t).into()));

        let body_subst = lam.body.subst(&lam.var, &arg_val.into());
        let next_step = EvalStep::app_abs(App::new(fun_val, arg_val), body_subst);
        steps.push(next_step);

        let body_res = body_subst.eval(env)?;
        let body_val = body_res.val();
        steps.extend(body_res.steps.into_iter());

        Ok(EvalTrace::new(steps, body_val))
    }
}
