use crate::{values::Lambda as LambdaVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Lambda, Term},
    types::Type,
};

impl<T, Ty> Eval for Lambda<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    LambdaVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LambdaVal::new(&self.var, self.annot, *self.body).into())
    }
}
