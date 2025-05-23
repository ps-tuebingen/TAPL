use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Loc, Term};

impl<T> Eval for Loc<T>
where
    T: Term + Eval,
    LocVal<T>: Into<<T as Term>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LocVal::new(self.loc).into())
    }
}
