use crate::{values::Raise as RaiseVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Raise, Term},
    types::Type,
};

impl<T, Ty> Eval for Raise<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    RaiseVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let exc_val = self.exception.eval(env)?;
        let raise_val =
            RaiseVal::<<T as Eval>::Value, Ty>::new(exc_val, self.cont_ty, self.exception_ty);
        Ok(raise_val.into())
    }
}
