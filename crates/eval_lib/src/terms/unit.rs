use crate::{values::Unit as UnitVal, Eval};
use common::errors::Error;
use syntax::terms::{Term, Unit};

impl<T> Eval for Unit<T>
where
    T: Term + Eval,
    UnitVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(UnitVal::new().into())
    }
}
