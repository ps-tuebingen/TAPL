use crate::Eval;
use syntax::{
    terms::{Term, UntypedLambda},
    values::UntypedLambda as UntypedLambdaVal,
};
use trace::EvalTrace;

impl<T> Eval for UntypedLambda<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    UntypedLambdaVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;
    type Term = T;

    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::new(
            vec![],
            UntypedLambdaVal::new(&self.var, *self.body),
        ))
    }
}
