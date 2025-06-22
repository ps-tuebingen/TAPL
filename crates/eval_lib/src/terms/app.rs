use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    store::Store,
    subst::SubstTerm,
    terms::{App, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for App<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Self: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<<T as Eval>::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let fun_res = self.fun.clone().eval(env)?;
        let fun_val = fun_res.val();
        let lam = fun_val.clone().into_lambda()?;

        let arg_res = self.arg.clone().eval(env)?;
        let arg_val: <T as Eval>::Value = arg_res.val();

        let body_subst = lam.body.subst(&lam.var, &arg_val.clone().into());
        let next_step = EvalStep::app_abs(App::new(fun_val, arg_val), body_subst.clone());

        let mut steps = fun_res.congruence(&move |t| App::new(t, *self.arg.clone()).into());
        steps.extend(arg_res.congruence(&move |t| App::new(*self.fun.clone(), t).into()));
        steps.push(next_step);

        let body_res = body_subst.eval(env)?;
        let body_val = body_res.val();
        steps.extend(body_res.steps);

        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, body_val))
    }
}
