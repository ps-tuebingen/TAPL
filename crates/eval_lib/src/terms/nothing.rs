use crate::{Eval, errors::EvalError};
use syntax::{
    store::Store,
    terms::{Nothing, Term},
    types::Type,
    values::Nothing as NothingVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Nothing<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Ty: Type,
    NothingVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(vec![], NothingVal::<T, Ty>::new(self.ty)))
    }
}
