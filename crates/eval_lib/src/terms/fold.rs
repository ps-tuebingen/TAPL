use crate::Eval;
use syntax::{
    terms::{Fold, Term},
    types::Type,
    values::Fold as FoldVal,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Fold<T, Ty>
where
    T: Term + Eval<Term = T>,
    Fold<T, Ty>: Into<T>,
    Ty: Type,
    FoldVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val: <T as Eval>::Value =
            FoldVal::<<T as Eval>::Value, Ty>::new(self.ty.clone(), term_val.clone()).into();
        let last_step = EvalStep::fold(Fold::new(term_val, self.ty.clone()), val.clone());
        let mut steps = term_res.congruence(&move |t| Fold::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
