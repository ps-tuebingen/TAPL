use crate::{values::Nothing as NothingVal, Eval};
use syntax::{
    terms::{Nothing, Term},
    types::Type,
};

impl<T, Ty> Eval for Nothing<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    NothingVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(NothingVal::<T, Ty>::new(self.ty).into())
    }
}
