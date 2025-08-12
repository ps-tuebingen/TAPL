use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    terms::{Term, TyLambda},
    values::TyLambda as TyLambdaVal,
};
use trace::EvalTrace;

impl<T> Eval for TyLambda<T>
where
    T: Term + Eval<Term = T>,
    TyLambdaVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            TyLambdaVal::new(&self.var, self.annot, *self.term),
        ))
    }
}
