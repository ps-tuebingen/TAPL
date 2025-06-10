use crate::Eval;
use syntax::{
    terms::{Nothing, Term},
    types::Type,
    values::Nothing as NothingVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Nothing<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    NothingVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(NothingVal::<T, Ty>::new(self.ty).into())
    }
}
