use crate::{values::Num as NumVal, Eval};
use common::errors::Error;
use syntax::terms::{Num, Term};

impl<T> Eval for Num<T>
where
    T: Term + Eval,
    NumVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NumVal::new(self.num).into())
    }
}
