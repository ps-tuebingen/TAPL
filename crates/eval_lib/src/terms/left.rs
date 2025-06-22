use crate::{errors::EvalError, Eval};
use syntax::{
    store::Store,
    terms::{Left, Term},
    types::Type,
    values::Left as LeftVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Left<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Left<T, Ty>: Into<T>,
    Ty: Type,
    LeftVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let left_res = self.left_term.eval(env)?;
        let left_val = left_res.val();
        let val = LeftVal::<<T as Eval>::Value, Ty>::new(left_val, self.ty.clone());
        let steps = left_res.congruence(&move |t| Left::new(t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }
}
