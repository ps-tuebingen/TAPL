use crate::{values::Exception as ExceptionVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> Eval for Exception<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    ExceptionVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(ExceptionVal::new(self.ty).into())
    }
}
