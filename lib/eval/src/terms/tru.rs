use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    terms::{Term, True},
    values::True as TrueVal,
};
use trace::EvalTrace;

impl<T> Eval for True<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    TrueVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(vec![], TrueVal::new()))
    }
}
