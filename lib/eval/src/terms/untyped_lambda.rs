use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
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

    type Term = T;

    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            UntypedLambdaVal::new(&self.var, *self.body),
        ))
    }
}
