use crate::{Eval, errors::EvalError};
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

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            vec![],
            LocVal::new(self.loc).into(),
        ))
    }
}
