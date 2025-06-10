use crate::Eval;
use syntax::{
    terms::{Term, TyLambda},
    values::TyLambda as TyLambdaVal,
};

impl<T> Eval for TyLambda<T>
where
    T: Term + Eval,
    TyLambdaVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(TyLambdaVal::new(&self.var, self.annot, *self.term).into())
    }
}
