use crate::Eval;
use syntax::{
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
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            vec![],
            NilVal::<T, Ty>::new(self.ty),
        ))
    }
}
