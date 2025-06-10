use crate::Eval;
use syntax::{
    terms::{Term, TyLambda},
    values::TyLambda as TyLambdaVal,
};
use trace::EvalTrace;

impl<T> Eval for TyLambda<T>
where
    T: Term + Eval,
    TyLambdaVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(TyLambdaVal::new(&self.var, self.annot, *self.term).into())
    }
}
