use crate::Eval;
use syntax::{
    terms::{Cast, Term},
    types::Type,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Cast<T, Ty>
where
    T: Term + Eval<Term = T>,
    Cast<T, Ty>: Into<T>,
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
        let inner_res = self.term.eval(env)?;
        let inner_val = inner_res.val();
        let last_step = EvalStep::cast(self.ty.clone(), inner_val.clone());
        let mut steps = inner_res.congruence(&move |t| Cast::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, inner_val))
    }
}
