use crate::{errors::EvalError, Eval};
use syntax::{
    eval_context::EvalContext,
    terms::{Nil, Term},
    types::Type,
    values::Nil as NilVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Nil<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Ty: Type,
    NilVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            vec![],
            NilVal::<T, Ty>::new(self.ty),
        ))
    }
}
