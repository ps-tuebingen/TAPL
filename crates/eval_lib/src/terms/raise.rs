use crate::{errors::EvalError, Eval};
use syntax::{
    store::Store,
    terms::{Raise, Term},
    types::Type,
    values::Raise as RaiseVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Raise<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Raise<T, Ty>: Into<T>,
    Ty: Type,
    RaiseVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let exc_res = self.exception.eval(env)?;
        let exc_val = exc_res.val();
        let raise_val = RaiseVal::<<T as Eval>::Value, Ty>::new(
            exc_val,
            self.cont_ty.clone(),
            self.exception_ty.clone(),
        );

        let steps = exc_res.congruence(&move |t| {
            Raise::new(t, self.cont_ty.clone(), self.exception_ty.clone()).into()
        });
        Ok(EvalTrace::new(steps, raise_val))
    }
}
