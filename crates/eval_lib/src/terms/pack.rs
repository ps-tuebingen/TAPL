use crate::{Eval, errors::EvalError};
use syntax::{
    store::Store,
    terms::{Pack, Term},
    types::Type,
    values::Pack as PackVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Pack<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Ty: Type,
    Pack<T, Ty>: Into<T>,
    PackVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();

        let val = PackVal::<<T as Eval>::Value, Ty>::new(
            self.inner_ty.clone(),
            term_val,
            self.outer_ty.clone(),
        );

        let steps = term_res.congruence(&move |t| {
            Pack::new(self.inner_ty.clone(), t, self.outer_ty.clone()).into()
        });

        Ok(EvalTrace::new(steps, val))
    }
}
