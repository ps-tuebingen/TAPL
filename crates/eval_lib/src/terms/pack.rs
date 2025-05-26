use crate::{values::Pack as PackVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Pack, Term},
    types::Type,
};

impl<T, Ty> Eval for Pack<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    PackVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(PackVal::<T>::new(self.inner_ty.clone(), term_val, self.outer_ty.clone()).into())
    }
}
