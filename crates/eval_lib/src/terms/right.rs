use crate::{values::Right as RightVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Right, Term},
    types::Type,
};

impl<T, Ty> Eval for Right<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    RightVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let right_val = self.right_term.eval(env)?;
        Ok(RightVal::<T>::new(right_val, self.ty.clone()).into())
    }
}
