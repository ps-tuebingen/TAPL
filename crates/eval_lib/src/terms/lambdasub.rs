use crate::{values::LambdaSub as LambdaSubVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{LambdaSub, Term},
    types::Type,
};

impl<T, Ty> Eval for LambdaSub<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    LambdaSubVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LambdaSubVal::new(&self.var, self.sup_ty, *self.body).into())
    }
}
