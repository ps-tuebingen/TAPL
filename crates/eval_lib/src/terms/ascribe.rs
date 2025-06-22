use crate::{errors::EvalError, Eval};
use syntax::{
    eval_context::EvalContext,
    terms::{Ascribe, Term},
    types::Type,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Ascribe<T, Ty>
where
    T: Term + Eval<Term = T> + From<Ascribe<T, Ty>> + From<<T as Eval>::Value>,
    Ty: Type,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let inner_res = self.term.clone().eval(env)?;
        let val = inner_res.val();
        let last_step = EvalStep::ascribe(Ascribe::new(val.clone(), self.ty.clone()), val.clone());
        let mut steps = inner_res.congruence(&move |t| Ascribe::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
