use crate::{values::Cons as ConsVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Cons, Term},
    types::Type,
};

impl<T, Ty, V> Eval for Cons<T, Ty>
where
    T: Term + Eval<Value = V>,
    Ty: Type,
    ConsVal<Ty, V>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let hd_val = self.head.eval(env)?;
        let tail_val = self.tail.eval(env)?;
        Ok(ConsVal::<Ty, V>::new(hd_val, tail_val, self.ty).into())
    }
}
