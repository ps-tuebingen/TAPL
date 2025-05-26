use crate::{values::TyLambda as TyLambdaVal, Eval};
use common::errors::Error;
use syntax::terms::{Term, TyLambda};

impl<T> Eval for TyLambda<T>
where
    T: Term + Eval,
    TyLambdaVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TyLambdaVal::new(&self.var, self.annot, *self.term).into())
    }
}
