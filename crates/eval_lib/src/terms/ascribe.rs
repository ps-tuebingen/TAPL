use crate::Eval;
use syntax::{
    terms::{Ascribe, Term},
    types::Type,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Ascribe<T, Ty>
where
    T: Term + Eval<Term = T> + From<Ascribe<T, Ty>> + From<<T as Eval>::Value>,
    Ty: Type,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let inner_res = self.term.clone().eval(env)?;
        let val = inner_res.val();
        let last_step = EvalStep::ascribe(Ascribe::new(val.clone(), self.ty.clone()), val.clone());
        let mut steps = inner_res.congruence(&move |t| Ascribe::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
