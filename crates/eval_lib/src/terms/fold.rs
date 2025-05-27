use crate::{values::Fold as FoldVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Fold, Term},
    types::Type,
};

impl<T, Ty> Eval for Fold<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    FoldVal<<T as Eval>::Value, Ty, T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(FoldVal::<<T as Eval>::Value, Ty, T>::new(self.ty, term_val).into())
    }
}
