use crate::{values::Left as LeftVal, Eval};
use common::errors::Error;
use syntax::terms::{Left, Term};

impl<T, Ty> Eval for Left<T, Ty>
where
    T: Term + Eval,
    LeftVal<T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let left_val = self.left_term.eval(env)?;
        Ok(LeftVal::<T>::new(left_val, self.ty).into())
    }
}
