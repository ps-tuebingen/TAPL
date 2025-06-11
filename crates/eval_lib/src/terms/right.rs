use crate::Eval;
use syntax::{
    terms::{Right, Term},
    types::Type,
    values::Right as RightVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Right<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Right<T, Ty>: Into<T>,
    Ty: Type,
    RightVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let right_res = self.right_term.eval(env)?;
        let right_val = right_res.val();
        let val = RightVal::<<T as Eval>::Value, Ty>::new(right_val, self.ty.clone());
        let steps = right_res.congruence(&move |t| Right::new(t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }
}
