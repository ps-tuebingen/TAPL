use crate::Eval;
use syntax::{
    store::Store,
    terms::{Loc, Term},
    values::Loc as LocVal,
};
use trace::EvalTrace;

impl<T> Eval for Loc<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    LocVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            vec![],
            LocVal::new(self.loc).into(),
        ))
    }
}
